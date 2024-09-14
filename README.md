
# **Rust-Bitcoin Sidechain**

This project implements a **Bitcoin-anchored sidechain** written in Rust, leveraging Bitcoin's **Proof-of-Work (PoW)** consensus mechanism through **merge mining**. The sidechain is fully anchored to Bitcoin Core, but without a two-way peg for asset transfer. Instead, it inherits the security of Bitcoin through periodic anchoring and merge mining.

## **Overview**

This sidechain is designed to:
- **Use Bitcoin's PoW consensus**: No separate consensus mechanism is implemented. The sidechain inherits Bitcoin's consensus via merge mining.
- **Be anchored to Bitcoin**: Sidechain blocks are periodically anchored in Bitcoin by including sidechain block headers in Bitcoin transactions.
- **Merge-mining compatibility**: Bitcoin miners can mine blocks on both Bitcoin and the sidechain simultaneously without needing to allocate additional resources.

## **Features**
- **Merge Mining (Auxiliary Proof-of-Work)**: The sidechain allows miners to mine Bitcoin and the sidechain simultaneously by reusing Bitcoin's PoW.
- **Bitcoin-anchored security**: Sidechain blocks are periodically anchored in Bitcoin blocks for additional security and tamper-proofing.
- **Rust implementation**: The sidechain is built using Rust, leveraging the `rust-bitcoin` crate for interaction with the Bitcoin protocol.
- **No asset peg**: The sidechain is non-peg, meaning there is no two-way transfer of assets between Bitcoin and the sidechain.

## **Components**
1. **Sidechain Block Structure**: The sidechain block is similar to a Bitcoin block but contains a reference to a Bitcoin block hash via the merge mining (AuxPoW) mechanism.
2. **AuxPoW (Auxiliary Proof-of-Work)**: This mechanism allows the sidechain to share Bitcoin's PoW. Miners submit a valid Bitcoin block and a sidechain block together.
3. **Anchoring Mechanism**: Periodically, sidechain block headers are committed to the Bitcoin blockchain using an OP_RETURN transaction, which acts as a checkpoint.

## **Architecture**

- **Bitcoin Core Integration**: The sidechain communicates with Bitcoin Core via the JSON-RPC API to retrieve Bitcoin block headers and submit anchor transactions.
- **Merge Mining**: Miners mine both Bitcoin and sidechain blocks using a single PoW computation.
- **Validation**: Sidechain nodes validate blocks based on Bitcoin’s PoW and ensure that the sidechain block is correctly linked to a valid Bitcoin block.

## **How It Works**
1. **Mining**: Bitcoin miners participate in mining both Bitcoin and sidechain blocks. The sidechain block contains the hash of a Bitcoin block, and the same PoW solution is used for both.
2. **Anchoring**: Periodically, the sidechain state is anchored in Bitcoin by committing block headers using an OP_RETURN transaction.
3. **Consensus**: The sidechain leverages Bitcoin’s consensus by reusing Bitcoin’s PoW via the AuxPoW mechanism. No new consensus mechanism is required.

## **Setup and Installation**

### **Prerequisites**
- **Rust**: Ensure Rust is installed on your machine. You can install Rust [here](https://www.rust-lang.org/tools/install).
- **Bitcoin Core**: Install and run Bitcoin Core (preferably in regtest mode for development).

### **Steps to Run the Sidechain**
1. **Clone the repository**:
   ```bash
   git clone https://github.com/AreaLayer/demo-sidechain.git
   cd sidechain
   ```

2. **Install dependencies**:
   The project uses the `rust-bitcoin` crate. Ensure you have the necessary Rust dependencies installed:
   ```bash
   cargo build
   ```

3. **Run Bitcoin Core in Regtest mode**:
   ```bash
   bitcoind -regtest -daemon
   ```

4. **Start the sidechain**:
   Run the sidechain node:
   ```bash
   cargo run --release
   ```

5. **Start merge-mining**:
   Set up your miner to perform merge mining on both Bitcoin and the sidechain.

### **Interacting with the Sidechain**
- **Check sidechain blocks**: The sidechain node validates new blocks mined via merge mining.
- **Submit anchors**: Anchor sidechain blocks periodically in Bitcoin using the OP_RETURN mechanism.

## **Development**

### **Modifying the Code**
- The sidechain block structure and the AuxPoW (merge mining) mechanism are located in the `src/block.rs` and `src/mining.rs` files, respectively.
- The anchoring mechanism is implemented in `src/anchor.rs`, which includes the logic for committing sidechain headers to Bitcoin.

### **Testing**
To test the sidechain in regtest mode:
- Use Bitcoin Core in regtest mode for quick block generation.
- Simulate mining on both the Bitcoin main chain and the sidechain.
- Verify that the sidechain blocks are anchored correctly in Bitcoin blocks.

## **Contributing**
Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs or feature requests.

## **License**
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## **Acknowledgments**
- **Bitcoin Core**: This project heavily relies on Bitcoin Core for anchoring and security.
- **rust-bitcoin**: The Rust library for Bitcoin protocol support.
