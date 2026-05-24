---
title: Noeracle
description: On-demand price oracle for Stellar.
---

Noeracle is an on-demand price oracle for Stellar.

A consumer fetches a freshly signed price attestation from Noeracle's off-chain service and bundles a verification operation into its own Stellar transaction. The Soroban contract verifies the publisher signature, checks freshness, and stores the price — so the consumer's application logic executes against a price signed within the last 2 seconds, not against pre-warmed on-chain state.

This is the freshness model that perpetual-DEX execution, lending liquidations, and oracle-priced AMM swaps require.

## Complement to Reflector

Noeracle is the complement to Reflector, not a replacement. Reflector pre-warms on-chain state on a cadence and serves free reads — optimal for displays, anchor UIs, stablecoin mint/redeem flows, and slow rebalancing. Noeracle delivers execution-time freshness bounded by the publisher signing cadence rather than ledger close time. Some protocols use both, per use case.

## Status

| | |
|---|---|
| Version | v0 — testnet prototype |
| Network | Stellar Testnet |
| Contract | [`CAYIP67UDVX5UPXGN3XDAWVIEFBAVG6G7LUESEOU3NUQKTWN55W34YBG`](https://stellar.expert/explorer/testnet/contract/CAYIP67UDVX5UPXGN3XDAWVIEFBAVG6G7LUESEOU3NUQKTWN55W34YBG) |
| Attestation service | `https://api.noeracle.org` |
| Freshness SLA | ≤2 seconds (testnet) |
| Source | [`noeracle/noeracle`](https://github.com/noeracle/noeracle) |

v0 runs a single self-operated signer and has not been independently audited. **Not for production capital.** A multi-publisher, audited mainnet version is in active development.

## Get started

- [Quickstart](/quickstart/) — integrate in under 10 lines
- [Integration guide](/integration/) — every pattern with code
- [SDK reference](/reference/sdk/) · [Contract reference](/reference/contract/)
- [Architecture](/architecture/) · [Threat model](/threat-model/) · [Roadmap](/roadmap/)
