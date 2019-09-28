## TX history to CSV

For a given address, find all its transactions and record whether
- it was an incoming or outgoing transaction
- how much interest was claimed in the event of an outgoing transaction returning change

There are multiple scenarios:
1. The address is not found in the Vin array, which means money came in from somewhere else.
    1. we only look at the amount received and record that as Incoming
2. The address is found in the Vin array, and the address was not found in the Vout array. This means KMD was sent, but no change went back to the address
    1. we only look at the amount spent in the Vin array
3. The address is found in both the Vin and the Vout, which can be one of two cases:
    1. KMD was sent to an external address and change was returned to original address (ordinary payment)
        1. with interest
        2. without interest
    2. This was an interest claim
    
### Build and run
First, you need Rust. Copy and paste in a terminal window to install it: 
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next, clone this repository, change directory into it and run `cargo build`. It will build the needed binary, which will be located in `~/csv_export/target/debug/csv_export`

Then, go to this directory and run the binary with the following 2 parameters (this assumes a komodo daemon is running with addressindex enabled):
```shell script
cd ~/csv_export/target/debug/
./csv_export <base58 address> <pubkey>
```

The application will run for a while and store the CSV in the same folder, called `komodo_tx.csv`.

### How this app works
The application assumes the following:
- a komodo daemon is running with addressindex enabled

   
This application records transactions in CSV, through the following layout:
```text
Timestamp,Description,Value,Txid
```

Given an address, the application scans the blockchain for all its transactions and records it using the Scenarios mentioned above, in the following way:
 
#### Scenario 1:
```text
1560542508,In,12.003555,3b36cbea54a130846db105a2a3ed6e303f3e7375f4ee37222a7bd15608db9608
```

#### Scenario 2:
```text
1558195169,Out,2408.83263947,e5e87ff78e7dea8ef01a53015caf7e09dd5217733a327efa918a6d22497a61eb
```

#### Scenario 3:
If the transaction contained interest, 2 transactions will be registered in the CSV:
- The outgoing amount
- The interest amount claimed

The 2 transactions have the same timestamp and txid:
```text
1563725207,Out,2895.0,a9c614439cf5df7b47302cfc58a3262862c4a2c4dcfd588872360b053c3deab3
1563725207,InterestClaim,4.09410936,a9c614439cf5df7b47302cfc58a3262862c4a2c4dcfd588872360b053c3deab3
```

If no interest was claimed, only an outgoing transaction will be included in the CSV:
```text
1568500368,Out,0.00010302,72fbc20551383083406ac7eb9faf7ec91ad04e71cc02c0bf7f3ce825ba406936
```