use web3::transports::Http;
use web3::types::{H160, U256};
use web3::Web3;
use rand::Rng;

async fn fuzz_smart_contract(contract_address: H160, function_signature: &str, num_tests: usize) {
    let http = Http::new("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID").unwrap();
    let web3 = Web3::new(http);

    for _ in 0..num_tests {
       
        let random_input: String = (0..10).map(|_| rand::random::<char>()).collect();

        
        let payload = format!("{}{}", function_signature, random_input);
        
        match web3.eth().send_transaction(payload).await {
            Ok(tx_hash) => {
                println!("Transaction successful with input: {}, tx hash: {:?}", random_input, tx_hash);
            }
            Err(e) => {
                println!("Transaction failed with input: {}, error: {:?}", random_input, e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let contract_address: H160 = "0xYourSmartContractAddress".parse().unwrap();
    let function_signature = "yourFunctionSignature";
    fuzz_smart_contract(contract_address, function_signature, 100).await;
}
