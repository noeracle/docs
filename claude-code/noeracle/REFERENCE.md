# noeracle deep reference

Auxiliary detail for the noeracle skill. Read this when SKILL.md doesn't answer the question.

## Architecture in one diagram

```
 5 exchanges (Coinbase, Binance, Kraken, OKX, Bybit)
        │  polled every ~500 ms
        ▼
 ┌──────────────────────────────┐
 │  Attestation service         │   api.noeracle.org
 │  • weighted aggregate         │   /v1/latest  (REST snapshot)
 │  • ed25519 sign 40-byte msg   │   /v1/stream  (SSE)
 └──────────────────────────────┘
        │  signed Attestation
        ▼
 ┌──────────────────────────────┐
 │  @noeracle/sdk               │   browser / Node
 │  Noeracle.fetchLatest()      │
 │  Fresh.updateArgs()  ◄───────┐
 │  Fresh.toUpdateOp()          │
 └──────────────────────────────┘
        │  6 ScVals
        ▼
 ┌──────────────────────────────┐
 │  Your consumer (Soroban)     │
 │  • verify (cross-contract)   │
 │  • use verified price        │
 └──────────────────────────────┘
        │  invoke_contract
        ▼
 ┌──────────────────────────────┐
 │  noeracle oracle (Soroban)   │   CAYIP67U...
 │  • ed25519 verify per asset  │
 │  • staleness ≤ 60 s          │
 │  • round_id monotonic         │
 │  • store latest per asset    │
 └──────────────────────────────┘
```

## Signed payload — 40 bytes, big-endian

```
offset  size  field
 0      8     asset tag (e.g. b"BTCUSD\0\0")
 8     16     price (i128, scaled by 1e7)
24      8     timestamp (u64, unix seconds)
32      8     round_id (u64, monotonic per asset)
```

The asset tag IS the on-chain identifier. The display name in the SDK (`"BTC/USD"`) is a client-side alias; it does not appear on the wire.

## Three-layer freshness

| Layer | Check | Default | Where |
| --- | --- | --- | --- |
| SDK | Reject attestation > N s old when received | 2 s | `freshnessLimitSeconds` in `NoeracleConfig` |
| Contract | Reject attestation > 60 s before current ledger time | 60 s | inside `update_batch_ed25519_args` |
| Round ID | Reject any round_id ≤ stored round_id per asset | always on | inside `update_batch_ed25519_args` (silent — lagging rounds don't revert) |

For Pattern B consumers, **the contract-level 60s is too coarse** for execution-time logic; layer your own `timestamp >= my_window_start` check.

## Contract reference

### Public entrypoints

```rust
// One-time setup, sets admin and registered publishers.
fn init(env: Env, admin: Address, publishers: Vec<BytesN<32>>) -> Result<(), Error>

// Admin-only rotation of the publisher set.
fn set_publishers(env: Env, publishers: Vec<BytesN<32>>) -> Result<(), Error>

// The pull-mode entrypoint. Verifies one ed25519 sig per asset, checks
// pubkey is in the registered publisher set, rejects rounds signed > 60s
// before ledger time, writes newer entries to temporary storage.
// Lagging rounds fail silently (no revert) so concurrent updaters don't
// step on each other.
fn update_batch_ed25519_args(
    env: Env,
    assets: Vec<BytesN<8>>,
    prices: Vec<i128>,
    timestamp: u64,
    round_id: u64,
    pubkey: BytesN<32>,
    sigs: Vec<BytesN<64>>,
) -> Result<(), Error>

// Free read of the latest verified price. Returns None if expired.
// Freshness is opportunistic — high-traffic assets stay warm.
fn get_price(env: Env, asset: BytesN<8>) -> Option<PriceEntry>
```

### Error codes

| Code | Variant | Trigger |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | second `init` call |
| 2 | `NotInitialized` | call before init |
| 3 | `BatchLengthMismatch` | misaligned vector lengths |
| 4 | `UnknownPublisher` | pubkey not in publisher set |
| 5 | `StalePrice` | age > 60 s vs ledger time |

### Storage TTLs

| Key | Storage | TTL | Notes |
| --- | --- | --- | --- |
| `PriceTemp(asset)` | Temporary | 360 ledgers (~30 min) | hot path |
| `PricePers(asset)` | Persistent | 60,480 ledgers (~3.5 days) | cold fallback |
| `Publishers` | Instance | Permanent | publisher pubkey set |

## SDK reference

### `Noeracle`

```ts
new Noeracle({
  network?: "testnet" | "mainnet",   // default "testnet"
  attestationUrl?: string,            // default "https://api.noeracle.org"
  contractId?: string,                // default oracle for toUpdateOp
  freshnessLimitSeconds?: number,     // default 2
})

fetchLatest(assets: string[]): Promise<Fresh>
subscribe(assets, onUpdate, onError?): Subscription
```

### `Fresh`

```ts
class Fresh {
  attestations: Attestation[]
  prices: PriceEntry[]
  price(asset: string): PriceEntry        // throws AssetUnavailableError
  updateArgs(): xdr.ScVal[]               // 6 ScVals to splat into Contract.call
  toUpdateOp(contractId?: string): xdr.Operation
}
```

### `PriceEntry`

```ts
{
  asset: string         // "BTC/USD"
  price: bigint         // i128 scaled by 1e7
  priceHuman: number    // decoded float
  timestamp: number     // unix seconds
  roundId: number       // monotonic per asset
  sources: number       // # exchanges behind the aggregate
}
```

### Errors

All extend `NoeracleError`:

| Class | When |
| --- | --- |
| `AttestationServiceError` | network / 5xx / unreachable |
| `AssetUnavailableError` | asset not in the snapshot |
| `StalePriceError` | attestation older than `freshnessLimitSeconds` |
| `InconsistentRoundError` | multi-asset fetch where rounds disagree |

## Threat model summary (v0)

**Defended:**
- stale-price replay (60s + monotonic round ID)
- unknown signer (publisher set check)
- cross-asset signature reuse (asset tag in signed message)
- lagging keeper (2s SDK reject)

**Open (mitigate at v1):**
- single publisher key compromise → full oracle compromise
- single attestation-service instance (Fly.io) → liveness SPOF
- low-liquidity asset manipulation (5 source aggregate)
- no independent audit
- MEV / front-running

Security contact: `security@noeracle.org`.

## Roadmap

- **v0** (now): single signer, testnet only
- **v1**: 3-of-5 ed25519 threshold (2 self + 3 external), audit, mainnet
- **v2**: FX, NAV, reserve attestations, TWAP
- **v3**: stake-backed publishers, equities/commodities, governance, paid feeds

## Useful patterns

### SSE subscription for liquidators / keepers

```js
const sub = oracle.subscribe(["BTC/USD"], (fresh) => {
  const p = fresh.price("BTC/USD");
  if (p.priceHuman < liquidationThreshold) {
    triggerLiquidation(fresh);  // bundle fresh into the liquidation tx
  }
});
// later: sub.close();
```

Reconnects automatically on transient errors. Good for long-running clients (UIs, keepers, market makers).

### Multi-asset single-snapshot

```js
const fresh = await oracle.fetchLatest(["BTC/USD", "ETH/USD"]);
// All entries share one round_id — safe for cross-asset spreads.
```

Throws `InconsistentRoundError` if the service returns disagreeing rounds (should not happen, but the SDK checks).
