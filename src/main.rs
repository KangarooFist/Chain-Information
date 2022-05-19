
#[macro_use]
extern crate serde_derive;

mod block_info;
mod block_address;
mod block_status;
mod block_tx;

//mod brings in other files 

use crate::{block_status::BlockStatus, block_info::tx_request};
use crate::block_address::BlockAddress;
use crate::block_tx::BlockTx;
use dotenv;
use std::{io, thread, time};

fn blockchain_info_app(address: &str) {
    let block_status: BlockStatus = block_info::block_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &block_status.blockbook.coin, &block_status.backend.chain);
    
    let block_address: BlockAddress = block_info::address_request(address);
    println!("\n\nAnalysing transaction for current ETH address {}", &block_address.address);

    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);

    println!("\nYou have a total of {} transactions", &block_address.txids.len());

    println!("\n Query these transactions? (Y/N)\n");

    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("Y") {
        println!("\nLooking up the Transactions");
        println!("{:#?}", &block_address.txids);
        thread::sleep(sleep_time);
    
        let mut balance: i32 = 0;
        for tx_id in &block_address.txids {

            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

            let block_tx: BlockTx = block_info::tx_request(&tx_id);

            let match_address = String::from(address);

            for tx in &block_tx.vin {
                if tx.addresses.contains(&match_address) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }
                
            for tx in &block_tx.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }

            }

            balance += &subtotal_vout - subtotal_vin;

            println!("-----------------------------------------------------");
            println!("TX ID:           {}", &block_tx.txid);
            println!("SATOSHIS IN:     {}", &subtotal_vout);
            println!("SATOSHIS OUT:    {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("-----------------------------------------------------");
        };

        println!("CURRENT BALANCE:     {}", &balance);
        println!("         IN ETH:     {}\n\n", balance as f32 * 0.00000001);
    }
}

fn main() {
    let entered_address = dotenv::var("Wallet").expect("Error reading Env Var");
    blockchain_info_app(&entered_address);
}