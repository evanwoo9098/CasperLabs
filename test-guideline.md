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

### Query examples

query balance
```console
casperlabs-client --host localhost balance -b 'block hash' -a 'public key of the account'
```

transfer from account to account
```console
casperlabs-client --host localhost \
transfer \
--amount 100000000 \
--payment-amount 100000000 \
--target-account 'account public key in base64 form' \
--private-key $KEY_DIR/validator-private.pem
```
