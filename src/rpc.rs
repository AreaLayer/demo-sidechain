use bitcoincore_rpc::{Auth, Client, RpcApi};

let rpc = Client::new("http://localhost:8332", Auth::UserPass("user".to_string(), "pass".to_string())).unwrap();
let best_block_hash = rpc.get_best_block_hash().unwrap();
