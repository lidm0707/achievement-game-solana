use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use anyhow::Result;
use std::rc::Rc;
use std::{env, path::PathBuf};

// import crate ของโปรแกรม Anchor
use achieve_game::Progress;
use achieve_game::accounts as accounts_module;
use achieve_game::id;
use achieve_game::instruction as instruction_module;

fn main() -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;

    let deadline: i64 = now + 60; // อีก 1 นาที

    // กำหนด URL ของ RPC (local/testnet/devnet)
    let url = Cluster::Custom(
        "http://127.0.0.1:8899".to_string(),
        "ws://127.0.0.1:8899".to_string(),
    );
    // หา home directory ของผู้ใช้
    let home = env::var("HOME").expect("Cannot find HOME environment variable");

    // สร้าง path แบบ dynamic
    let keypair_path: PathBuf = [home.as_str(), ".config/solana/id.json"].iter().collect();

    // โหลด keypair
    let payer = read_keypair_file(&keypair_path).expect(&format!(
        "Failed to read keypair file at {:?}",
        keypair_path
    ));
    // โหลด keypair ของ payer
    // let payer = read_keypair_file("~/.config/solana/id.json").expect("Failed to read keypair file");

    let client = Client::new(url, Rc::new(&payer));

    // same build api request by token
    let owner = client.program(id())?;

    // กำหนด game_id
    // let game_id: u64 = 42;
    let game_id: u64 = 43;
    let server_id: u64 = 123;
    let provider_id: u64 = 456;
    let pub_key = payer.pubkey();
    let event_id: u64 = 789;

    // Derive PDA ของ game
    let (game_pda, _bump) = Pubkey::find_program_address(
        &[
            b"game",
            owner.payer().as_ref(),
            // pub_key.as_ref(),
            &game_id.to_le_bytes(),
            &server_id.to_le_bytes(),
            &provider_id.to_le_bytes(),
            &event_id.to_le_bytes(),
        ],
        &owner.id(),
    );

    // --- เช็คว่ามี account อยู่แล้วหรือไม่ ---
    let game_account_result = owner.account::<Progress>(game_pda);

    if game_account_result.is_err() {
        println!("Game PDA ไม่พบ, กำลัง initialize...");

        let tx_init = owner
            .request()
            .accounts(accounts_module::Initialize {
                game: game_pda,
                owner: owner.payer(),
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(instruction_module::Initialize {
                game_id,
                server_id,
                provider_id,
                deadline,
            })
            .send()?;

        println!("Initialize tx signature: {}", tx_init);
    } else {
        println!("Game PDA มีอยู่แล้ว");
    }

    // --- เรียก ongoing ---
    let tx_ongoing = owner
        .request()
        .accounts(accounts_module::OnGoing {
            game: game_pda,
            owner: pub_key,
        })
        .args(instruction_module::Ongoing {})
        .send()?;

    println!("Ongoing tx signature: {}", tx_ongoing);

    // --- Fetch account ล่าสุด ---
    let game_account: Progress = owner.account(game_pda)?;
    // println!(
    //     "Game data - game_id: {}, score: {} , pub_key: {}",
    //     game_account.game_id, game_account.score, game_account.owner
    // );

    println!(
        "Game data - game_id: {}, score: {}, deadline: {}",
        game_account.game_id, game_account.score, game_account.deadline
    );
    Ok(())
}
