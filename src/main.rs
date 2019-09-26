extern crate komodo_rpc_client;

use komodo_rpc_client::{KomodoRpcApi, TransactionId};
use komodo_rpc_client::Client;
use komodo_rpc_client::arguments::AddressList;
use std::ops::Mul;

// Address: RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH
// Pubkey: 03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d

fn main() {
    let client = Client::new_komodo_client().unwrap();

    const ADDRESS: &str = "RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH";
    const FCOIN: f64 = 100_000_000.0;

    let txns = client.get_address_tx_ids(&AddressList::from(ADDRESS)).unwrap();

    /*

    /////////////////
    // Scenario 1
    /////////////////
    const TXID: &str = "41440090b484e74ba62b03f5af075898c1e377be0e0b4a0333b6dd5d5836c421";

    let raw_tx = client.get_raw_transaction_verbose(
        TransactionId::from_hex(TXID).unwrap()).unwrap();

    let mut sum_vout = 0.0;
    for vout in raw_tx.vout {
        if vout.script_pubkey.addresses.contains(&String::from(ADDRESS)) {
            sum_vout += vout.value;
        }
    }
    println!("Received only: {}", sum_vout);

    */

    /*

    /////////////////
    // Scenario 2
    /////////////////
    const TXID: &str = "ef12a191b6804a793197f97d3d8faa2fdfdf74db32fb2b8ed2dd4e13b83889be";

    let raw_tx = client.get_raw_transaction_verbose(
        TransactionId::from_hex(TXID).unwrap()).unwrap();

    let mut sum_vin = 0.0;
    for vin in raw_tx.vin {
        // find out whether RLAG was an input in the transaction:
        if vin.script_sig.hex.contains("03d768320d35afdb24944fbab98af1e01a7826f55d937b477575f2df6cbe3d897e") {
            println!("jse was input");
            // get the amount that was spent:
            let raw_tx_previous = client.get_raw_transaction_verbose(vin.txid).unwrap();
            for vout in raw_tx_previous.vout {
                if vout.script_pubkey.addresses.contains(&"RMrsxLvmfRyn9dWdbT6T2PYjz88Hxgxjse".to_string()) {
                    sum_vin += vout.value;
                }
            }
        }
    }
    println!("Outgoing only: {}", sum_vin);

    */

    /*

    /////////////////
    // Scenario 3
    /////////////////
    let mut sum_vin: f64 = 0.0;
    for vin in raw_tx.vin {
        // find out whether RLAG was an input in the transaction:
        if vin.script_sig.hex.contains("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
            println!("RLAG was input");
            // get the amount that was spent:
            let raw_tx_previous = client.get_raw_transaction_verbose(vin.txid).unwrap();
            for vout in raw_tx_previous.vout {
                if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) {
                    sum_vin += vout.value;
                }
            }
        }
    }
    println!("Sum vin: {}", sum_vin);

    // find out what the sum is of the Vout:
    let mut sum_vout: f64 = 0.0;
    let mut sum_address: f64 = 0.0;

    for vout in raw_tx.vout {
        sum_vout += vout.value;
        if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) {
            sum_address += vout.value;
        }
    }

    println!("Sum vout: {}", sum_vout);

    // the interest based on `vout - vin`
    println!("Difference (interest): {}", (( (sum_vout * 100000000.0) as u32 - (sum_vin  * 100000000.0) as u32) as f64) / 100000000.0);

    // now we need to find out how much of that was returned to RLAG
    println!("Returned to RLAG: {}", sum_address);

    */

    let mut scenarios: Vec<TX> = vec![];

    for tx_str in txns.0 {
        let mut raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&tx_str).unwrap()).unwrap();
        let tx_in = 0;
        let mut spent = false;
        let mut receive = false;

        for vin in &raw_tx.vin {
            // if `.contains()` is used, multisig address inputs will be counted too
            if vin.script_sig.hex.ends_with("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
                spent = true;
            }
        }

        for vout in &raw_tx.vout {
            if vout.script_pubkey.addresses.contains(&String::from(ADDRESS)) {
                receive = true;
            }
        }


        match (spent, receive) {
            (false, true) => {
//                println!("Scenario 1: {}", &tx_str);

                let raw_tx = client.get_raw_transaction_verbose(
                    TransactionId::from_hex(&tx_str).unwrap()).unwrap();

                let mut sum_vout = 0;
                for vout in raw_tx.vout {
                    if vout.script_pubkey.addresses.contains(&String::from(ADDRESS)) {
                        sum_vout += ((vout.value * FCOIN) + 0.5) as u64;
                    }
                }

                // add to Vec
                scenarios.push(TX {
                    description: MoneyFlow::In,
                    txid: tx_str,
                    value: (sum_vout as f64) / FCOIN,
                    timestamp: raw_tx.time.unwrap()
                })
            }, // incoming
            (true, false) => {
//                println!("Scenario 2: {}", &tx_str);

                let raw_tx = client.get_raw_transaction_verbose(
                    TransactionId::from_hex(&tx_str).unwrap()).unwrap();

                let mut sum_vin = 0;
                for vin in raw_tx.vin {
                    // find out with which utxos RLAG was an input in the transaction:
                    if vin.script_sig.hex.contains("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
                        // get the amount that was spent:
                        let raw_tx_previous = client.get_raw_transaction_verbose(vin.txid).unwrap();
                        for vout in raw_tx_previous.vout {
                            if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) {
                                sum_vin += ((vout.value * FCOIN) + 0.5) as u64;
                            }
                        }
                    }
                }
                scenarios.push(TX {
                    description: MoneyFlow::Out,
                    txid: tx_str,
                    value: (sum_vin as f64) / FCOIN,
                    timestamp: raw_tx.time.unwrap()
                })
            }, // outgoing only
            (true, true) => {
//                println!("Scenario 3 :{}" , &tx_str);

                let mut interest_claim = true;
                let mut sum_vin: u64 = 0;

//                if tx_str.eq("917cb5bc00d35ce0e135521da9697e6699905ba5536261f80b7a05307be716b8") {
                    raw_tx.vin.sort_by(|a, b|a.txid.to_string().cmp(&b.txid.to_string()));
                    raw_tx.vin.dedup_by(|a, b| a.txid.to_string().eq(&b.txid.to_string()));
                    for vin in raw_tx.vin {
                        // find out whether RLAG was an input in the transaction:
//                        dbg!(&vin);
                        if vin.script_sig.hex.ends_with("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
//                        println!("RLAG was input");
                            // get the amount that was spent:
//                            println!("vin.txid: {}", &vin.txid);
//                            println!("vin.vout: {}", &vin.vout);
                            let raw_tx_previous = client.get_raw_transaction_verbose(vin.txid).unwrap();
                            // when the previous tx was an interest claim, possibly 2 or more utxos were in that tx as output
                            // therefore the vin.vout number needs to be equal to the vout.n of the previous tx.
                            for vout in raw_tx_previous.vout {
                                if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) && vout.n == vin.vout {
//                                    println!("vout.value: {}", vout.value);
//                                    dbg!(&vout);
//                                    println!("vout.value: {}", vout.value);
                                    sum_vin += ((vout.value * FCOIN) + 0.5) as u64;
                                }
                            }
                        }
                    }
//                    println!("Sum vin: {}", sum_vin);

                    // find out what the sum is of the Vout:
                    let mut sum_vout: u64 = 0;
                    let mut sum_address: u64 = 0;
                    let mut sum_other_address: u64 = 0;

                    for vout in raw_tx.vout {
//                        dbg!(&vout);
//                        println!("vout.value: {}", vout.value);
                        let sats = ((vout.value * FCOIN) + 0.5) as u64;
//                        dbg!(vout.value);
//                        dbg!(sats);
                        sum_vout += sats;
//                        dbg!(sum_vout);

                        if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) {
                            sum_address += (vout.value.mul(FCOIN) + 0.5) as u64;
                        } else {
                            interest_claim = false; // there is another address, so not a self-sending interest claim!
                            sum_other_address += ((vout.value * FCOIN) + 0.5) as u64;
                        }
                    }
//                    dbg!(sum_vout);

                    if interest_claim {
//                        println!("interest claim: {}: {} - {}", &tx_str, sum_vout, sum_vin);
                        scenarios.push(TX {
                            description: MoneyFlow::InterestClaim,
                            txid: tx_str,
                            value: (sum_vout - sum_vin) as f64 / FCOIN,
                            timestamp: raw_tx.time.unwrap()
                        });
                    } else {

                        // KMD moves out
                        scenarios.push(TX {
                            description: MoneyFlow::Out,
                            txid: tx_str.clone(),
                            value: (sum_other_address as f64) / FCOIN,
                            timestamp: raw_tx.time.unwrap()
                        }); // todo still losing interest in on this one
                        // interest was possibly claimed, need to add to income
                        // any negative subtraction is healthy tx fee
//                        println!("non-interest claim: {}: vout.{} - vin.{}", &tx_str, sum_vout, sum_vin);
                        if (sum_vout as i64) - (sum_vin as i64) < 0 {
                            println!("txfee: {}", (sum_vin as i64) - (sum_vout as i64))
                        } else {
                            println!("interest in tx: {}", (sum_vout as i64) - (sum_vin as i64));
                            scenarios.push(TX {
                                description: MoneyFlow::InterestClaim,
                                txid: tx_str,
                                value: (sum_vout - sum_vin) as f64 / FCOIN,
                                timestamp: raw_tx.time.unwrap()
                            });
                        }
                    }
//                }


//                println!("Sum vout: {}", sum_vout);

                // the interest based on `vout - vin`
//                println!("Difference (interest): {}", (( (sum_vout * 100000000.0) as u32 - (sum_vin  * 100000000.0) as u32) as f64) / 100000000.0);

                // now we need to find out how much of that was returned to RLAG
//                println!("Returned to RLAG: {}", sum_address);

            }, // outgoing with change AND / OR interest claim
            _ => {}
        }
    }

//    dbg!(scenarios);
}

#[derive(Debug)]
struct TX {
    description: MoneyFlow,
    txid: String,
    value: f64,
    timestamp: u64
}

#[derive(Debug)]
enum MoneyFlow {
    In,
    Out,
    InterestClaim
}



