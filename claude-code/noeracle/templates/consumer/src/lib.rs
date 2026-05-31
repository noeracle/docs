#![no_std]

//! Pattern B noeracle consumer template.
//!
//! Replace `do_thing_with_price` with your business logic. The shape is the
//! same as any Soroban contract — the only noeracle-specific parts are:
//!   1. The six forwarded oracle args in the entrypoint signature.
//!   2. The `env.invoke_contract::<()>(&oracle, ...)` call inside the body.
//!   3. The `timestamp >= window_start` check that prevents replay of a
//!      recently-signed but pre-window attestation (noeracle's 60s window
//!      is not the same as your contract's time window).

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal,
    Symbol, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    BatchEmpty = 3,
    WrongAsset = 4,
    PriceBeforeWindow = 5,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Oracle,
    AssetTag,
}

#[contract]
pub struct MyApp;

#[contractimpl]
impl MyApp {
    pub fn init(env: Env, oracle: Address, asset_tag: BytesN<8>) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Oracle) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::AssetTag, &asset_tag);
        Ok(())
    }

    /// Replace this with your business logic. Args (in order):
    ///   - user: the caller, who must auth.
    ///   - assets/prices/timestamp/round_id/pubkey/sigs: the six values
    ///     returned by `Fresh.updateArgs()` in the JS SDK, forwarded verbatim
    ///     to the oracle.
    ///   - window_start: your business-logic time threshold; the verified
    ///     price MUST have been signed at or after this point.
    pub fn do_thing_with_price(
        env: Env,
        user: Address,
        assets: Vec<BytesN<8>>,
        prices: Vec<i128>,
        timestamp: u64,
        round_id: u64,
        pubkey: BytesN<32>,
        sigs: Vec<BytesN<64>>,
        window_start: u64,
    ) -> Result<i128, Error> {
        user.require_auth();

        if assets.is_empty() {
            return Err(Error::BatchEmpty);
        }

        let expected_tag: BytesN<8> = env
            .storage()
            .instance()
            .get(&DataKey::AssetTag)
            .ok_or(Error::NotInitialized)?;
        if assets.get_unchecked(0) != expected_tag {
            return Err(Error::WrongAsset);
        }
        if timestamp < window_start {
            return Err(Error::PriceBeforeWindow);
        }

        let oracle: Address = env
            .storage()
            .instance()
            .get(&DataKey::Oracle)
            .ok_or(Error::NotInitialized)?;

        // Atomic verify. If sig / publisher / staleness fails inside the
        // oracle, the whole transaction reverts here.
        env.invoke_contract::<()>(
            &oracle,
            &Symbol::new(&env, "update_batch_ed25519_args"),
            vec![
                &env,
                assets.into_val(&env),
                prices.clone().into_val(&env),
                timestamp.into_val(&env),
                round_id.into_val(&env),
                pubkey.into_val(&env),
                sigs.into_val(&env),
            ],
        );

        let verified_price = prices.get_unchecked(0);
        // -------- YOUR BUSINESS LOGIC HERE --------
        // verified_price is i128 scaled by 1e7. For example:
        //   if verified_price > strike { ... }
        //   collateral_value_usd = (collateral * verified_price) / 1e7
        // ------------------------------------------
        let _ = (user, round_id); // silence unused warnings in skeleton
        Ok(verified_price)
    }
}

#[cfg(test)]
mod test;
