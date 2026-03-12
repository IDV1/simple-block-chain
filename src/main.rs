mod header;
mod hash;
mod errors;
mod block;
mod block_chain;

use crate::block_chain::chain::Chain;

fn main() {
    let mut init = Chain::new();
    let trs_data = "林先生挣到了5个比特币".to_string();
    init.add_block(trs_data);
    init.print_info();
}
