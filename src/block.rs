extern crate bitcoin;
use bitcoin::blockdata::block::{Block, BlockHeader};
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::network::constants::Network;

fn create_block() -> Block {
    let header = BlockHeader {
        version: 0x20000000,
        prev_blockhash: Default::default(),
        merkle_root: Default::default(),
        time: 1234567890,
        bits: 0x1d00ffff,
        nonce: 0,
    };

    let txs: Vec<Transaction> = vec![];  // No transactions for now

    Block {
        header,
        txdata: txs,
    }
}
