use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use anyhow::Result;
use std::rc::Rc;
use std::{env, path::PathBuf};

// --- achieve_game ---
use achieve_game::Progress;
use achieve_game::accounts as game_accounts;
use achieve_game::id;
use achieve_game::instruction as game_ix;

// --- reward ---
use reward::Reward;
use reward::accounts as reward_accounts;
use reward::id as reward_pid;
use reward::instruction as reward_ix;

fn main() -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    let deadline: i64 = now + 60;

    // RPC URL
    // let url = Cluster::Custom(
    //     "http://127.0.0.1:8899".to_string(),
    //     "ws://127.0.0.1:8899".to_string(),
    // );
    let url = Cluster::Devnet;

    // keypair
    let home = env::var("HOME").expect("Cannot find HOME environment variable");
    let keypair_path: PathBuf = [home.as_str(), ".config/solana/id.json"].iter().collect();
    let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair");
    let pub_key = payer.pubkey();

    // client
    let client = Client::new(url, Rc::new(payer));

    // program clients
    let owner = client.program(id())?;
    let reward_prog = client.program(reward_pid())?;
    println!("owner {:?}", pub_key);
    // IDs
    let game_id: u64 = 43;
    let server_id: u64 = 123;
    let provider_id: u64 = 456;
    let event_id: u64 = 789;

    // --- derive reward PDA ---
    let (reward_pda, _bump) = Pubkey::find_program_address(
        &[b"reward", pub_key.as_ref(), &event_id.to_le_bytes()],
        &reward_prog.id(),
    );

    // --- check reward account ---
    let reward_account_result = reward_prog.account::<Reward>(reward_pda);

    if reward_account_result.is_err() {
        println!("Reward PDA ไม่พบ, กำลัง initialize...");

        let tx_reward = reward_prog
            .request()
            .accounts(reward_accounts::Initialize {
                authority: pub_key,
                reward: reward_pda,
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(reward_ix::Initialize {
                event_id,
                amount: 100,
            })
            .send()?;

        println!("InitializeReward tx signature: {}", tx_reward);
    } else {
        println!("Reward PDA มีอยู่แล้ว");
    }

    // --- derive game PDA ---
    let (game_pda, _bump) = Pubkey::find_program_address(
        &[
            b"game",
            pub_key.as_ref(), // 👈 ต้อง match owner ที่ส่งเข้า ctx
            // owner.payer().as_ref(),
            // pub_key.as_ref(), // 👈 ใช้ payer pubkey
            &game_id.to_le_bytes(),
            &server_id.to_le_bytes(),
            &provider_id.to_le_bytes(),
            &event_id.to_le_bytes(),
        ],
        &pub_key,
    );
    println!("Derived game PDA = {}", game_pda);

    // --- check game account ---
    let game_account_result = owner.account::<Progress>(game_pda);

    if game_account_result.is_err() {
        println!("Game PDA ไม่พบ, กำลัง initialize...");

        let tx_init = owner
            .request()
            .accounts(game_accounts::Initialize {
                game: game_pda,
                // owner: owner.payer(),
                owner: pub_key, // 👈 consistent กับ seeds

                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(game_ix::Initialize {
                game_id,
                server_id,
                provider_id,
                deadline,
                event_id,
            })
            .send()?;

        println!("InitializeGame tx signature: {}", tx_init);
    } else {
        println!("Game PDA มีอยู่แล้ว");
    }

    // --- ongoing ---
    let tx_ongoing = owner
        .request()
        .accounts(game_accounts::OnGoing {
            game: game_pda,
            owner: pub_key,
            reward: reward_pda,               // ✅ เพิ่ม
            reward_program: reward_prog.id(), // ✅ เพิ่ม
        })
        .args(game_ix::Ongoing { event_id }) // ✅ ไม่ใช่ Ongoing {}
        .send()?;

    println!("Ongoing tx signature: {}", tx_ongoing);

    // --- fetch latest account ---
    let game_account: Progress = owner.account(game_pda)?;
    println!(
        "Game data - game_id: {}, score: {}, deadline: {}",
        game_account.game_id, game_account.score, game_account.deadline
    );

    Ok(())
}
