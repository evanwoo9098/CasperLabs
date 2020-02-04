## step 1
run execution engine
```
casperlabs-engine-grpc-server ~/.casperlabs/.casper-node.sock
```
## step 2
run node

```
casperlabs-node run \
    --casper-standalone \
    --tls-key $KEY_DIR/node.key.pem \
    --tls-certificate $KEY_DIR/node.certificate.pem \
    --casper-validator-private-key-path $KEY_DIR/validator-private.pem \
    --casper-validator-public-key-path $KEY_DIR/validator-public.pem
```

## step 3

### deploy WASM

```
casperlabs-client \
    --host localhost \
    deploy \
    --from $VALIDATOR_ID \
    --session $TEST_CONTRACT_DIR/counter_define.wasm \
    --payment $TEST_CONTRACT_DIR/standard_payment.wasm \
    --payment-args '[{"name":"amount", "value": {"big_int": {"value":"100000000", "bit_width": 512}}}]' \
    --private-key $KEY_DIR/validator-private.pem
```

```
casperlabs-client --host localhost propose
```

```
casperlabs-client --host localhost show-blocks
```

```
casperlabs-client \
--host localhost \
query-state \
-t address \
--block-hash "put the block hash" \
-k $VALIDATOR_ID -p "counter/count"
```

#### deploy stored contract

```
casperlabs-client \
    --host localhost \
    deploy \
    --from $VALIDATOR_ID \
    --session-hash "hash" \
    --session-args '[{"name": "method", "value": {"string_value": "inc"}}]'
    --payment $TEST_CONTRACT_DIR/standard_payment.wasm \
    --payment-args '[{"name":"amount", "value": {"big_int": {"value":"100000000", "bit_width": 512}}}]' \
    --private-key $KEY_DIR/validator-private.pem
```

### Query examples

query balance
```console
casperlabs-client --host localhost balance -b 'block hash' -a 'public key of the account'
```

### Transfer from account to account

```console
casperlabs-client --host localhost \
transfer \
--amount 100000000 \
--payment-amount 100000000 \
--target-account '3b6ebe77cff843183961bc5023b381939645e0f33d559c3f09a0abafcabedcbe' \
--private-key $KEY_DIR/validator-private.pem
```

```console
casperlabs-client \
    --host localhost \
    deploy \
    --from $VALIDATOR_ID \
    --session $TEST_CONTRACT_DIR/transfer_to_account.wasm \
    --session-args '[{"name": "target_address", "value": {"bytes_value": "6e88ed546646f6613224351c1695bd583dad138999b397bffa4d541fe99ebc59"}}, {"name": "amount", "value": {"long_value": 100000000}}]' \
    --payment $TEST_CONTRACT_DIR/standard_payment.wasm \
    --payment-args '[{"name":"amount", "value": {"big_int": {"value":"1000000", "bit_width": 512}}}]' \
    --private-key $KEY_DIR/validator-private.pem
```

### Bond, Unbond

#### bond
```
casperlabs-client \
    --host localhost \
    deploy \
    --from $VALIDATOR_ID \
    --session $TEST_CONTRACT_DIR/bonding.wasm \
    --session-args '[{"name": "amount", "value": {"long_value": 1000000}}]' \
    --payment $TEST_CONTRACT_DIR/standard_payment.wasm \
    --payment-args '[{"name":"amount", "value": {"big_int": {"value":"20000000", "bit_width": 512}}}]' \
    --private-key .accounts/account1/validator-private.pem
```

#### unbond

casperlabs-client \
    --host localhost \
    deploy \
    --from $VALIDATOR_ID \
    --session $TEST_CONTRACT_DIR/unbonding.wasm \
    --session-args '[{"name": "amount", "value": {"long_value": 1000000}}]' \
    --payment $TEST_CONTRACT_DIR/standard_payment.wasm \
    --payment-args '[{"name":"amount", "value": {"big_int": {"value":"20000000", "bit_width": 512}}}]' \
    --private-key .accounts/account1/validator-private.pem
