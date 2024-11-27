use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signature::{Keypair, Signer, EncodableKey}, transaction::Transaction, program_pack::Pack,
};
use spl_token::{
    instruction::initialize_mint, state::Mint, ID as TOKEN_PROGRAM_ID,
};

fn main() {
    // Set up Solana client
    let rpc_url = "https://api.devnet.solana.com"; // Use testnet/devnet for testing
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for the token mint
    let mint = Keypair::new();
    println!("Token Mint Address: {}", mint.pubkey());

    // Load your wallet keypair from a file
    let payer = Keypair::read_from_file("/users/Ashwin/solana-wallet.json")
        .expect("Failed to load wallet keypair");

    // Create an associated token account for the mint authority
    let mint_authority = payer.pubkey();

    // Rent-exempt balance for mint account
    let mint_account_balance = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .expect("Failed to get minimum rent balance");

    // Create the mint account
    let create_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_account_balance,
        Mint::LEN as u64,
        &TOKEN_PROGRAM_ID,
    );

    // Initialize the mint
    let init_mint_ix = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint.pubkey(),
        &mint_authority,
        None,
        6, // Decimal places
    )
    .expect("Failed to create initialize_mint instruction");

    // Create and send transaction
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        client.get_latest_blockhash().expect("Failed to get blockhash"),
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Transaction signature: {}", signature);

    println!(
        "Congratulations! Your token {} is live with mint authority: {}",
        mint.pubkey(),
        mint_authority
    );
}



// Wrote new keypair to /Users/ashwin/solana-wallet.json
// =====================================================================
// pubkey: B1MLm6QBkVi5QVD7QeaWCGDE3hPpJ87iVABkWcGt3VAG
// =====================================================================
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// fruit asset print key wisdom odor slender place noble hip sudden left


// Token Mint Address: 9u7FVq7Y5ba1qxvwMPeV2J76CTN3k6VGFJnu4qPo1KZ1
// Transaction signature: sLbFY6ycVyiEBPC5SB9XiYpucMo2AQZLuaa4ifTHp7Y3nrnYv7LKGoaPsRtkiSrny9BQGJBSUVo1UjttXg31zei
// Congratulations! Your token 9u7FVq7Y5ba1qxvwMPeV2J76CTN3k6VGFJnu4qPo1KZ1 is live with mint authority: B1MLm6QBkVi5QVD7QeaWCGDE3hPpJ87iVABkWcGt3VAG
