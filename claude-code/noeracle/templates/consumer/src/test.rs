#![cfg(test)]

use super::*;
use soroban_sdk::{
    contract as sdk_contract, contractimpl as sdk_contractimpl,
    testutils::Address as _,
    vec, Address, BytesN, Env, Vec,
};

/// Mock noeracle oracle for unit tests — accepts anything. Signature
/// verification is the real oracle's job; here we just want to exercise
/// that our consumer correctly invokes it and uses the returned price.
#[sdk_contract]
pub struct MockOracle;

#[sdk_contractimpl]
impl MockOracle {
    pub fn update_batch_ed25519_args(
        _env: Env,
        _assets: Vec<BytesN<8>>,
        _prices: Vec<i128>,
        _timestamp: u64,
        _round_id: u64,
        _pubkey: BytesN<32>,
        _sigs: Vec<BytesN<64>>,
    ) {
    }
}

// CRITICAL: this is the on-chain tag for BTC/USD. NOT "BTC/USD\0".
const BTCUSD_TAG: [u8; 8] = *b"BTCUSD\0\0";

fn setup() -> (Env, MyAppClient<'static>, Address, BytesN<8>) {
    let env = Env::default();
    env.mock_all_auths();
    let oracle_id = env.register(MockOracle, ());
    let app_id = env.register(MyApp, ());
    let client = MyAppClient::new(&env, &app_id);
    let tag = BytesN::from_array(&env, &BTCUSD_TAG);
    client.init(&oracle_id, &tag);
    let user = Address::generate(&env);
    (env, client, user, tag)
}

fn oracle_args(
    env: &Env,
    tag: &BytesN<8>,
    price: i128,
    ts: u64,
    round: u64,
) -> (
    Vec<BytesN<8>>,
    Vec<i128>,
    u64,
    u64,
    BytesN<32>,
    Vec<BytesN<64>>,
) {
    (
        vec![env, tag.clone()],
        vec![env, price],
        ts,
        round,
        BytesN::from_array(env, &[0u8; 32]),
        vec![env, BytesN::from_array(env, &[0u8; 64])],
    )
}

#[test]
fn happy_path_returns_verified_price() {
    let (env, client, user, tag) = setup();
    let (a, p, t, r, k, s) = oracle_args(&env, &tag, 80_000i128 * 10_000_000, 1_000, 1);
    let price = client.do_thing_with_price(&user, &a, &p, &t, &r, &k, &s, &1_000);
    assert_eq!(price, 80_000i128 * 10_000_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")] // WrongAsset
fn rejects_wrong_asset() {
    let (env, client, user, _tag) = setup();
    let eth = BytesN::from_array(&env, b"ETHUSD\0\0");
    let (a, p, t, r, k, s) = oracle_args(&env, &eth, 80_000i128 * 10_000_000, 1_000, 1);
    client.do_thing_with_price(&user, &a, &p, &t, &r, &k, &s, &1_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")] // PriceBeforeWindow
fn rejects_pre_window_price() {
    let (env, client, user, tag) = setup();
    let (a, p, t, r, k, s) = oracle_args(&env, &tag, 80_000i128 * 10_000_000, 900, 1);
    client.do_thing_with_price(&user, &a, &p, &t, &r, &k, &s, &1_000);
}
