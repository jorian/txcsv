extern crate komodo_rpc_client;
extern crate serde;
extern crate csv;

use komodo_rpc_client::{KomodoRpcApi, TransactionId};
use komodo_rpc_client::Client;
use komodo_rpc_client::arguments::AddressList;
use std::io;
use serde::Serialize;

const ADDRESS: &str = "RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH";
const FCOIN: f64 = 100_000_000.0;
// Address: RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH
// Pubkey: 03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d

fn main() {
    let client = Client::new_komodo_client().unwrap();
    let txns = client.get_address_tx_ids(&AddressList::from(ADDRESS)).unwrap();

    let mut scenarios: Vec<TX> = vec![];

    for tx_str in txns.0 {
        let mut raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&tx_str).unwrap()).unwrap();
        let mut spent = false;
        let mut receive = false;

        // check which Scenario this transaction fits:
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
            (false, true) => scenario_1(&tx_str, &client, &mut scenarios),
            (true, false) => scenario_2(&tx_str, &client, &mut scenarios),
            (true, true) => scenario_3(&tx_str, &client, &mut scenarios),
            _ => println!("nothing spent, nothing received. you have found yourself in a weird state.")
        }
    }

    write_to_csv(scenarios)
}

fn write_to_csv(scenarios: Vec<TX>) {
    let mut wrtr = csv::Writer::from_writer(io::stdout());

    for tx in scenarios {
        wrtr.serialize(tx).unwrap();
    }
}

fn scenario_1(tx_str: &str, client: &Client, scenarios: &mut Vec<TX>) {
    let raw_tx = client.get_raw_transaction_verbose(
        TransactionId::from_hex(&tx_str).unwrap()).unwrap();

    let mut sum_vout = 0;
    for vout in raw_tx.vout {
        if vout.script_pubkey.addresses.contains(&String::from(ADDRESS)) {
            sum_vout += ((vout.value * FCOIN) + 0.5) as u64;
        }
    }

    scenarios.push(TX {
        description: MoneyFlow::In,
        txid: tx_str.to_string(),
        value: (sum_vout as f64) / FCOIN,
        timestamp: raw_tx.time.unwrap(),
    })
}

fn scenario_2(tx_str: &str, client: &Client, scenarios: &mut Vec<TX>) {
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
        txid: tx_str.to_string(),
        value: (sum_vin as f64) / FCOIN,
        timestamp: raw_tx.time.unwrap(),
    })
}

fn scenario_3(tx_str: &str, client: &Client, scenarios: &mut Vec<TX>) {
    // we assume interest is being claimed, unless there is another address in the vout
    let mut interest_claim = true;
    let mut sum_vin: u64 = 0;

    raw_tx.vin.sort_by(|a, b| a.txid.to_string().cmp(&b.txid.to_string()));
    raw_tx.vin.dedup_by(|a, b| a.txid.to_string().eq(&b.txid.to_string()));
    for vin in raw_tx.vin {
        // find out whether RLAG was an input in the transaction:
        if vin.script_sig.hex.ends_with("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
            // get the amount that was spent:
            let raw_tx_previous = client.get_raw_transaction_verbose(vin.txid).unwrap();
            // when the previous tx was an interest claim, possibly 2 or more utxos were in that tx as output
            // therefore the vin.vout number needs to be equal to the vout.n of the previous tx.
            for vout in raw_tx_previous.vout {
                if vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) && vout.n == vin.vout {
                    sum_vin += ((vout.value * FCOIN) + 0.5) as u64;
                }
            }
        }
    }

    // find out what the sum is of the Vout:
    let mut sum_vout: u64 = 0;
    let mut sum_going_out: u64 = 0;

    for vout in raw_tx.vout {
        sum_vout += ((vout.value * FCOIN) + 0.5) as u64;

        if !vout.script_pubkey.addresses.contains(&ADDRESS.to_string()) {
            interest_claim = false; // there is another address, so not a self-sending interest claim!
            sum_going_out += ((vout.value * FCOIN) + 0.5) as u64;
        }
    }

    // if the transaction was a self-sent interest claim, we only record the amount that was claimed:
    if interest_claim {
        scenarios.push(TX {
            description: MoneyFlow::InterestClaim,
            txid: tx_str.to_string(),
            value: (sum_vout - sum_vin) as f64 / FCOIN,
            timestamp: raw_tx.time.unwrap(),
        });
    } else {
        // KMD moves out and change, possibly including interest, was sent back
        // We first record the outgoing amount:
        scenarios.push(TX {
            description: MoneyFlow::Out,
            txid: tx_str.to_string(),
            value: (sum_going_out as f64) / FCOIN,
            timestamp: raw_tx.time.unwrap(),
        });
        // interest was possibly claimed, need to record as income
        // any negative subtraction is healthy tx fee (mostly with amounts transferred < 10, where no interest is involved)
        if (sum_vout as i64) - (sum_vin as i64) < 0 {
            println!("txfee: {}", (sum_vin as i64) - (sum_vout as i64))
        // if the total of all vout is more than the total of all vin, interest is claimed:
        } else {
            println!("interest in tx: {}", (sum_vout as i64) - (sum_vin as i64));
            scenarios.push(TX {
                description: MoneyFlow::InterestClaim,
                txid: tx_str.to_string(),
                value: (sum_vout - sum_vin) as f64 / FCOIN,
                timestamp: raw_tx.time.unwrap(),
            });
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TX {
    timestamp: u64,
    txid: String,
    value: f64,
    description: MoneyFlow,
}

#[derive(Debug, Serialize)]
enum MoneyFlow {
    In,
    Out,
    InterestClaim,
}



