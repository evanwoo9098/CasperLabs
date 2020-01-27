#!/bin/bash

if [ ${PWD##*/} != "CasperLabs" ]; then
  echo "error : run this script in CasperLabs repo root"
  exit 1
fi

rm -rf $HOME/.casperlabs varlist

set -eux

# build node
sbt node/universal:stage

# build client
sbt client/universal:stage

# build EE
cd execution-engine
make setup
make build

# build contracts
declare -a TARGET_CONTRACTS=(
  "mint-install"
  "pos-install"
  "counter-call"
  "counter-define"
  "standard-payment"
  "transfer-to-account"
  "bonding"
  "unbonding"
)

declare -a SYSTEM_WASMS=(
  "mint_install.wasm"
  "pos_install.wasm"
)

declare -a TEST_WASMS=(
  "counter_call.wasm"
  "counter_define.wasm"
  "standard_payment.wasm"
  "transfer_to_account.wasm"
  "bonding.wasm"
  "unbonding.wasm"
)

for pkg in "${TARGET_CONTRACTS[@]}"; do
  make build-contract-rs/$pkg
done

# copy contracts
# create dir for test contract
GENESIS_DIR="$HOME/.casperlabs/chainspec/genesis"
mkdir -p "$GENESIS_DIR"

TEST_CONTRACT_DIR="$HOME/.casperlabs/test-contract"
mkdir -p "$TEST_CONTRACT_DIR"

# copy system contracts
for wasm in "${SYSTEM_WASMS[@]}"; do
  cp "./target/wasm32-unknown-unknown/release/$wasm" "$GENESIS_DIR"
done

# copy test contracts
for wasm in "${TEST_WASMS[@]}"; do
  cp "./target/wasm32-unknown-unknown/release/$wasm" "$TEST_CONTRACT_DIR"
done

# key generation
cd ..
KEY_DIR="$HOME/.casperlabs/keys"
mkdir -p "$KEY_DIR"
./hack/key-management/docker-gen-keys.sh "$KEY_DIR"

# create bond.txt
(cat $KEY_DIR/validator-id; echo ",50000000000,1000000") > "$GENESIS_DIR/accounts.csv"

VALIDATOR_ID="$(cat $KEY_DIR/validator-id-hex)"

# create varlist
echo "export PATH=\"$PATH:$(pwd)/node/target/universal/stage/bin:\
$(pwd)/client/target/universal/stage/bin:\
$(pwd)/execution-engine/target/debug\"" >> ./varlist
echo "export TEST_CONTRACT_DIR=\"$TEST_CONTRACT_DIR\"" >> ./varlist
echo "export GENESIS_DIR=\"$GENESIS_DIR\"" >> ./varlist
echo "export KEY_DIR=\"$KEY_DIR\"" >> ./varlist
echo "export VALIDATOR_ID=\"$VALIDATOR_ID\"" >> ./varlist

echo "RUN source varlist"
