extern crate komodo_rpc_client;

use komodo_rpc_client::{KomodoRpcApi, TransactionId};
use komodo_rpc_client::Client;
use komodo_rpc_client::arguments::AddressList;

// Address: RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH
// Pubkey: 03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d

fn main() {
    let client = Client::new_komodo_client().unwrap();

    const ADDRESS: &str = "RLAGwyipfdDCPNDLhWgyYeu3d7BPsNoXGH";

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

    for tx_str in txns.0 {
        let raw_tx = client.get_raw_transaction_verbose(TransactionId::from_hex(&tx_str).unwrap()).unwrap();
        let tx_in = 0;
        let mut spent = false;
        let mut receive = false;
        for vin in raw_tx.vin {
            if vin.script_sig.hex.contains("03127be86a9a59a1ad13c788cd50c5ad0089a1fb05caa11aef6cc19cfb60d8885d") {
                spent = true;
            }
        }

        for vout in raw_tx.vout {
            if vout.script_pubkey.addresses.contains(&String::from(ADDRESS)) {
                receive = true;
            }
        }


        match (spent, receive) {
            (false, true) => println!("Scenario 1: {}", &tx_str), // incoming
            (true, false) => println!("Scenario 2: {}", &tx_str), // outgoing only
            (true, true) => println!("Scenario 3 :{}" , &tx_str), // outgoing with change AND / OR interest claim
            _ => {}
        }
    }
}

struct TX {
    description: MoneyFlow,
    txid: String,
    value: f32,
    timestamp: String
}

enum MoneyFlow {
    In,
    Out,
}



