use bitcoin::hashes::sha256d::Hash;

struct SidechainBlock{
    version: u32,
    prev_blockhash: Hash,
    merkle_root: Hash,
    time: u32,
    bits: u32,
    nonce: u32,
    taproot_merkele_branch: u128,
    aux_pow: AuxPoW,  // Data structure for AuxPoW
}

struct AuxPoW {
    coinbase_transaction: Vec<u8>,      // Coinbase transaction referencing sidechain work
    merkle_branch: Vec<Hash>,           // Merkle branch linking sidechain to Bitcoin block
}
