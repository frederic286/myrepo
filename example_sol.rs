use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
    signer::keypair::read_keypair_file,
    transaction::{Transaction, TransactionError},
};
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction::create_account,
    system_program,
};

fn main() {
    // Connecting to the Solana test network
    let rpc_url = "https://testnet.solana.com".to_string();
    let rpc_client = RpcClient::new(rpc_url);

   // Loading a key pair for signing transactions
    let keypair = read_keypair_file("path/to/keypair.json").unwrap();

    // Creating a new key pair for the wallet
    let new_account = Keypair::new();

    // Defining an instruction to create a new purse core
    let instruction = create_account(
        &keypair.pubkey(),
        &new_account.pubkey(),
        1,  // The number of SOLs to be transferred to the new wallet
        solana_sdk::loader_instruction::id(),
    );

    // Create a transaction with this instruction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));

    // Signing the transaction
    let (recent_blockhash, fee_calculator) = rpc_client.get_recent_blockhash().unwrap();
    transaction.sign(&[&keypair, &new_account], recent_blockhash);

    // Send the transaction to the network
    let result = rpc_client.send_transaction_with_config(
        &transaction,
        RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
        },
    );

    match result {
        Ok(signature) => println!("Transaction sent: {}", signature),
        Err(TransactionError::RpcError(error)) => println!("Transaction failed: {}", error),
        Err(TransactionError::SignaturesMissing) => println!("Transaction failed: signatures missing"),
        Err(TransactionError::InstructionError(_, _)) => println!("Transaction failed: instruction error"),
        Err(TransactionError::InvalidTransaction) => println!("Transaction failed: invalid transaction"),
    }
}
