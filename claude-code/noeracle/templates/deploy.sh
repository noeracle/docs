#!/usr/bin/env bash
# One-shot build + deploy + init + smoke-test for a noeracle consumer.
# Run from the contract crate directory (where Cargo.toml lives).
#
#   bash deploy.sh                       # uses 'hackathon' identity, BTC/USD
#   IDENTITY=foo ASSET_TAG=4254435553440000 bash deploy.sh
set -euo pipefail

IDENTITY=${IDENTITY:-hackathon}
ASSET_TAG=${ASSET_TAG:-4254435553440000}    # BTCUSD\0\0
ORACLE=${ORACLE:-CAYIP67UDVX5UPXGN3XDAWVIEFBAVG6G7LUESEOU3NUQKTWN55W34YBG}
NETWORK=${NETWORK:-testnet}

STELLAR=${STELLAR:-stellar}
if ! "$STELLAR" --version | grep -qE 'stellar 2[6-9]\.'; then
  echo "WARN: stellar CLI is older than 26.x. If deploy fails with 'xdr value invalid',"
  echo "      run: cargo install stellar-cli --locked && stellar config migrate"
fi

# Get the crate name from Cargo.toml.
CRATE=$(awk -F'"' '/^name *=/ {print $2; exit}' Cargo.toml)
WASM=target/wasm32v1-none/release/${CRATE}.wasm

echo "[1/5] cargo build --target wasm32v1-none --release"
cargo build --target wasm32v1-none --release

if ! "$STELLAR" keys address "$IDENTITY" >/dev/null 2>&1; then
  echo "[2/5] creating + funding identity '$IDENTITY'"
  "$STELLAR" keys generate --network "$NETWORK" --fund "$IDENTITY"
else
  echo "[2/5] identity '$IDENTITY' exists"
fi

echo "[3/5] deploying $WASM"
CONSUMER=$("$STELLAR" contract deploy --wasm "$WASM" --source "$IDENTITY" --network "$NETWORK")
echo "       consumer = $CONSUMER"

echo "[4/5] init"
"$STELLAR" contract invoke --id "$CONSUMER" --source "$IDENTITY" --network "$NETWORK" -- \
  init --oracle "$ORACLE" --asset_tag "$ASSET_TAG"

echo "[5/5] done"
echo ""
echo "  CONSUMER_ID=$CONSUMER"
echo "  ADMIN_SECRET=\$($STELLAR keys show $IDENTITY)"
echo ""
echo "Run the demo from your client dir:"
echo "  ADMIN_SECRET=\$($STELLAR keys show $IDENTITY) CONSUMER_ID=$CONSUMER node demo.mjs"
