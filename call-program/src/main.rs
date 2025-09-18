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

    let client = Client::new(url, Rc::new(payer));
    let program = client.program(id())?;

    // กำหนด game_id
    // let game_id: u64 = 42;
    let game_id: u64 = 43;

    // Derive PDA ของ game
    let (game_pda, _bump) = Pubkey::find_program_address(
        &[b"game", program.payer().as_ref(), &game_id.to_le_bytes()],
        &program.id(),
    );

    // --- เช็คว่ามี account อยู่แล้วหรือไม่ ---
    let game_account_result = program.account::<Progress>(game_pda);

    if game_account_result.is_err() {
        println!("Game PDA ไม่พบ, กำลัง initialize...");

        let tx_init = program
            .request()
            .accounts(accounts_module::Initialize {
                game: game_pda,
                user: program.payer(),
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(instruction_module::Initialize { game_id })
            .send()?;

        println!("Initialize tx signature: {}", tx_init);
    } else {
        println!("Game PDA มีอยู่แล้ว");
    }

    // --- เรียก ongoing ---
    let tx_ongoing = program
        .request()
        .accounts(accounts_module::OnGoing { game: game_pda })
        .args(instruction_module::Ongoing {})
        .send()?;

    println!("Ongoing tx signature: {}", tx_ongoing);

    // --- Fetch account ล่าสุด ---
    let game_account: Progress = program.account(game_pda)?;
    println!(
        "Game data - game_id: {}, score: {}",
        game_account.game_id, game_account.score
    );

    Ok(())
}
