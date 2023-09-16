# hack-zurich-2023-storage
Casper Hashmap contract for participants to store proofs / hashs in.


# Build the wasm binary
```
cargo build --release --target wasm32-unknown-unknown
```

# Add data to the "watches" dictionary using rust client

```
casper-client put-deploy --node-address http://88.99.4.9:7777 --session-hash hash-7417524af6078182df8c7cec4f5b45c7657da0ac0fc02d91c757fd9a28ae282d \
 --payment-amount 100000000000 --chain-name casper-test --session-arg "key:String='SOME_KEY'" "value:String='SOME_VALUE_OR_HASH'" \
 --secret-key ./kdd_secret_key.pem \
 --session-entry-point submit

```

Replace SOME_KEY with a key and SOME_VALUE_OR_HASH with a value.

