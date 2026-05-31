// Generic noeracle consumer demo.
//
//   npm install
//   ADMIN_SECRET=S... CONSUMER_ID=C... node demo.mjs
//
// Env overrides:
//   ASSET=BTC/USD          # the feed to fetch
//   METHOD=do_thing_with_price
//   ORACLE=C...            # different oracle (rare)
//   RPC_URL=...            # different Stellar RPC

import { Noeracle } from "@noeracle/sdk";
import {
  Contract, Keypair, Networks, TransactionBuilder,
  nativeToScVal, rpc, scValToNative,
} from "@stellar/stellar-sdk";

const RPC_URL = process.env.RPC_URL || "https://soroban-testnet.stellar.org";
const NETWORK = Networks.TESTNET;
const ORACLE = process.env.ORACLE || "CAYIP67UDVX5UPXGN3XDAWVIEFBAVG6G7LUESEOU3NUQKTWN55W34YBG";
const CONSUMER = process.env.CONSUMER_ID;
const ASSET = process.env.ASSET || "BTC/USD";
const METHOD = process.env.METHOD || "do_thing_with_price";
const ADMIN_SECRET = process.env.ADMIN_SECRET;

if (!ADMIN_SECRET) {
  console.error("Set ADMIN_SECRET to a funded testnet secret (starts with 'S').");
  process.exit(1);
}
if (!CONSUMER) {
  console.error("Set CONSUMER_ID to your deployed consumer contract address.");
  process.exit(1);
}

const server = new rpc.Server(RPC_URL);
const admin = Keypair.fromSecret(ADMIN_SECRET);
const oracle = new Noeracle({ network: "testnet" });

console.log(`[fetch ] fetching ${ASSET}...`);
const fresh = await oracle.fetchLatest([ASSET]);
const p = fresh.price(ASSET);
console.log(`[fetch ] ${ASSET}=$${p.priceHuman.toFixed(2)}  round=${p.roundId}  signed=${Math.floor(Date.now()/1000) - p.timestamp}s ago`);

const account = await server.getAccount(admin.publicKey());
const op = new Contract(CONSUMER).call(
  METHOD,
  nativeToScVal(admin.publicKey(), { type: "address" }),
  ...fresh.updateArgs(),
  nativeToScVal(BigInt(Math.floor(Date.now() / 1000)), { type: "u64" }),  // window_start = now
);
const tx = new TransactionBuilder(account, {
  fee: "200000",
  networkPassphrase: NETWORK,
}).addOperation(op).setTimeout(60).build();

const prepared = await server.prepareTransaction(tx);
prepared.sign(admin);
const sent = await server.sendTransaction(prepared);
console.log(`[submit] ${sent.hash} (${sent.status})`);

let landed;
for (let i = 0; i < 30; i++) {
  await new Promise((r) => setTimeout(r, 2000));
  landed = await server.getTransaction(sent.hash);
  if (landed.status !== "NOT_FOUND") break;
}
console.log(`[landed] ${landed.status}`);
if (landed.status === "SUCCESS" && landed.returnValue) {
  const result = scValToNative(landed.returnValue);
  console.log("[result]", typeof result === "bigint" ? result.toString() : result);
} else if (landed.status !== "SUCCESS") {
  console.error(JSON.stringify(landed, null, 2));
  process.exit(1);
}
