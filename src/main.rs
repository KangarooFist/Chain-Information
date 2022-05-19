
#[macro_use]
extern crate serde_derive;

mod block_info;
mod block_address;
mod block_status;
mod block_tx;

//mod brings in other files 

use crate::block_status::BlockStatus;
use crate::block_address::BlockAddress;
use crate::block_tx::BlockTx;

fn main() {
    let block_status: BlockStatus = block_info::block_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &block_status.blockbook.coin, &block_status.backend.chain);
}
