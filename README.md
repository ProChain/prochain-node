# Prochain Node

An advertising blockchain implemented by ProChain on Parity Substrate, ready for hacking.

# Building

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build Prochain:

```
cargo build --release
```

Ensure you have a fresh start if updating from another version:
```
./target/release/prochain purge-chain
```

To start up the Prochain node, run:
```
./target/release/prochain \
  --chain ./testnet/v0.9.0.raw.json \
  --name NodeName \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

# Settings

1) Open [Polkadot UI](https://polkadot.js.org/apps/#/explorer) . 

2) Go to *Settings*, open *Developer* tab. Insert in textbox description of types (copy&paste from here) and Save it.


```json
{
  "ExternalAddress": {
    "btc": "Vec<u8>",
    "eth": "Vec<u8>",
    "eos": "Vec<u8>"
  },
  "LockedRecords": {
    "locked_time": "Moment",
    "locked_period": "Moment",
    "locked_funds": "Balance",
    "rewards_ratio": "u64",
    "max_quota": "u64"
  },
  "UnlockRecords": {
    "unlock_time": "Moment",
    "unlock_funds": "Balance"
  },
  "MetadataRecord": {
    "address": "AccountId",
    "superior": "Hash",
    "creator": "AccountId",
    "did_ele": "Vec<u8>",
    "locked_records": "Option<LockedRecords<Balance, Moment>>",
    "unlock_records": "Option<UnlockRecords<Balance, Moment>>",
    "social_account": "Option<Hash>",
    "subordinate_count": "u64",
    "group_name": "Option<Vec<u8>>",
    "external_address": "ExternalAddress"
  },
  "AdsMetadata": {
    "advertiser": "Vec<u8>",
    "topic": "Vec<u8>",
    "total_amount": "Balance",
    "surplus": "Balance",
    "gas_fee_used": "Balance",
    "single_click_fee": "Balance",
    "create_time": "Moment",
    "period": "Moment"
  },
  "EventHTLC": {
    "eth_contract_addr": "Vec<u8>",
    "htlc_block_number": "BlockNumber",
    "event_block_number": "BlockNumber",
    "expire_height": "u32",
    "random_number_hash": "Vec<u8>",
    "swap_id": "Hash",
    "event_timestamp": "u64",
    "htlc_timestamp": "u64",
    "sender_addr": "Vec<u8>",
    "sender_chain_type": "HTLCChain",
    "receiver_addr": "AccountId",
    "receiver_chain_type": "HTLCChain",
    "recipient_addr": "Vec<u8>",
    "out_amount": "Balance",
    "event_type": "HTLCType"
  },
  "HTLCChain": {
    "_enum": [
      "ETHMain",
      "PRA"
    ]
  },
  "HTLCStates": {
    "_enum": [
      "INVALID",
      "OPEN",
      "COMPLETED",
      "EXPIRED"
    ]
  },
  "EventLogSource": {
    "event_name": "Vec<u8>",
    "event_url": "Vec<u8>"
  },
  "HTLCType": {
    "_enum": [
      "HTLC",
      "Claimed",
      "Refunded"
    ]
  }
}
```
# Validating on Prochain
Welcome to the official, in-depth Prochain guide to validating. We're happy that you're interested in validating on Prochain and we'll do our best to provide in-depth documentation on the process below.

This document contains all the information one should need to start validating on Prochain using the polkadot-js/apps user interface. We will start with how to setup one's node and proceed to how to key management. To start, we will use the following terminology of keys for the guide:

* stash - the stash keypair is where most of your funds should be located. It can be kept in cold storage if necessary.
* controller - the controller is the keypair that will control your validator settings. It should have a smaller balance, e.g. 10-100 PRA
* session - the 4 session keypairs are hot keys that are stored on your validator node. They do not need to have balances.

## Requirements
1. You should have balances in your stash (ed25519 or sr25519) and controller (ed25519 or sr25519) accounts.
2. You will need to additionally add the --validator flag to run a validator node.
3. You should have a wallet, such as the polkadot-js extension, installed in your browser with the stash and controller keypairs. If you don't have it, get it [here](https://github.com/polkadot-js/extension) . 

## Create a stake
Go to the Staking tab, and select Account actions at the top. Click on New stake.

Select your controller and stash accounts. Enter how much of your stash balance you would like to stake. Leave a few PRA free, or you will be unable to send transactions from the account.

You can also choose where your validator rewards are deposited (to the stash or the controller) and whether rewarded PRA should be automatically re-staked.

Sign and send the transaction

## Set your session keys, using rotateKeys
Click on Set Session Keys on the stake you just created above.

Go to the command line where your validator is running (e.g. SSH into the server, etc.) and enter this command. It will tell your validator to generate a new set of session keys:
```
curl -H 'Content-Type: application/json' --data '{ "jsonrpc":"2.0", "method":"author_rotateKeys", "id":1 }' localhost:9933
```

The output should look like this:
```
{"jsonrpc":"2.0","result":"0x0ca0fbf245e4abca3328f8bba4a286d6cb1796516fcc68864cab580f175e6abd2b9107003014fc6baab7fd8caf4607b34222df62f606248a8a592bcba86ff9eec6e838ae8eb757eb77dffc748f1443e60c4f7617c9ea7905f0dd09ab758a8063","id":1}
```

Copy the hexadecimal key from inside the JSON object, and paste it into the web interface.

Sign and send the transaction.

## Start validating
You should now see a Validate button on the stake. Click on it, and enter the commission you would like to charge as a validator. Sign and send the transaction.

You should now be able to see your validator in the Next up section of the staking tab.

At the beginning of the next era, if there are open slots and your validator has adequate stake supporting it, your validator will join the set of active validators and automatically start producing blocks. Active validators receive rewards at the end of each era. Slashing also happens at the end of each era.

Is your validator not producing blocks?

* Check that it is part of the active validator set. You will need to wait until your validator rotates in. this may take longer depending on whether there are free slots.
* Check that it is running with the --validator flag.
* Ensure your session keys are set correctly. Use curl to rotate your session keys again, and then send another transaction to the network to set the new keys.

## Stop validating
If you would like to stop validating, you should use the Stop Validating button on your stake, to send a chill transaction. It will take effect when the next validator rotation happens, at which point you can shut down your validator.

Once you have stopped validating, you can send a transaction to unbond your funds. You can then redeem your unbonded funds after the unbonding period has passed.

# Development

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units. Give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet). You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `QmQZ8TjTqeDj3ciwr93EJ95hxfDsb9pEYDizUAbWpigtQN`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmQZ8TjTqeDj3ciwr93EJ95hxfDsb9pEYDizUAbWpigtQN \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024 \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.
