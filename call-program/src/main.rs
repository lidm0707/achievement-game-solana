use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use anyhow::Result;
use quest;
use quest::accounts as quest_accounts;
use quest::instruction as quest_instruction;
use quest::state::quest_info::QuestInfo;
use reward::accounts as reward_accounts;
use reward::instruction as reward_instruction;
use reward::state::reward::Reward;
use std::rc::Rc;
use std::{env, path::PathBuf};

fn main() -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    let deadline: i64 = now + 60000;

    // RPC URL (local validator)
    let url = Cluster::Custom(
        "http://127.0.0.1:8899".to_string(),
        "ws://127.0.0.1:8899".to_string(),
    );

    // keypair
    let home = env::var("HOME").expect("Cannot find HOME environment variable");
    let keypair_path: PathBuf = [home.as_str(), ".config/solana/id.json"].iter().collect();
    let payer = read_keypair_file(&keypair_path).expect("Failed to read keypair");
    let admin_pub_key = payer.pubkey();

    // client
    let client = Client::new(url, Rc::new(payer));

    // program clients
    let quest = client.program(quest::id())?;
    let reward = client.program(reward::id())?;
    println!("admin {:?}", admin_pub_key);

    // IDs
    let quest_id: u64 = 43;
    let user_id: u64 = 1;
    let server_id: u64 = 123;
    let provider_id: u64 = 456;
    let max_score: u64 = 2;

    // --- derive reward PDA ---
    // // admin_pub_key.as_ref(),
    let (reward_pda, _bump) =
        Pubkey::find_program_address(&[b"reward", &quest_id.to_le_bytes()], &reward.id());

    // --- check reward account ---
    let reward_account_result = reward.account::<Reward>(reward_pda);

    if reward_account_result.is_err() {
        println!("Reward PDA ไม่พบ, กำลัง initialize...");

        let tx_reward = reward
            .request()
            .accounts(reward_accounts::CreateReward {
                reward: reward_pda,
                server_admin: admin_pub_key,
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(reward_instruction::CreateReward {
                quest_id,
                amount: 100,
            })
            .send()?;

        println!("InitializeReward tx signature: {}", tx_reward);
    } else {
        println!("Reward PDA มีอยู่แล้ว");
    }

    // --- derive game PDA ---
    //             b"quest",
    // server_admin.key().as_ref(),
    // &quest_id.to_le_bytes().as_ref(),
    // &user_id.to_le_bytes().as_ref(),
    // &server_id.to_le_bytes().as_ref(),
    // &provider_id.to_le_bytes().as_ref(),
    let (quest_pda, _bump) = Pubkey::find_program_address(
        &[
            b"quest",
            admin_pub_key.as_ref(),
            &quest_id.to_le_bytes().as_ref(),
            &user_id.to_le_bytes().as_ref(),
            &server_id.to_le_bytes().as_ref(),
            &provider_id.to_le_bytes().as_ref(),
        ],
        &quest.id(),
    );
    println!("Derived game PDA = {}", quest_pda);

    // --- check game account ---
    let quest_account_result = quest.account::<QuestInfo>(quest_pda);

    if quest_account_result.is_err() {
        println!("Game PDA ไม่พบ, กำลัง initialize...");

        let tx_init = quest
            .request()
            .accounts(quest_accounts::CreateQuest {
                quest_score: quest_pda,
                server_admin: admin_pub_key,
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(quest_instruction::CreateQuest {
                quest_id,
                user_id,
                server_id,
                provider_id,
                deadline,
                max_score,
            })
            .send()?;
        // ctx: Context<CreateQuest>,
        // quest_id: u64,
        // user_id: u64,
        // server_id: u64,
        // provider_id: u64,
        // deadline: i64,
        // max_score: u64,
        println!("InitializeGame tx signature: {}", tx_init);
    } else {
        println!("Game PDA มีอยู่แล้ว");
    }

    // --- loop ongoing ---
    for i in 0..3 {
        let tx = quest
            .request()
            .accounts(quest_accounts::MakeQuest {
                quest_score: quest_pda,
                server_admin: admin_pub_key,
                system_program: anchor_lang::solana_program::system_program::id(),
            })
            .args(quest_instruction::MakeQuest {
                quest_id,
                user_id,
                server_id,
                provider_id,
            })
            .send()?;

        println!(" - Loop {}/3 Ongoing tx signature: {}", (i + 1), tx);
    }

    // --- fetch game account ---
    let quest_account: QuestInfo = quest.account(quest_pda)?;
    println!(
        "Game data - game_id: {}, score: {}, deadline: {}, max_score: {}",
        quest_account.quest_id,
        quest_account.score,
        quest_account.deadline,
        quest_account.max_score
    );

    // --- fetch reward before update ---
    let reward_account: Reward = reward.account(reward_pda)?;
    println!(
        "Reward data - quest_id: {}, amount: {}",
        reward_account.quest_id, reward_account.amount
    );

    // --- manual update reward (direct call) ---
    println!("\n--- Manual Update Reward ---");
    // server_admin
    // system_program
    // quest_pda
    // quest_program
    let tx_update = reward
        .request()
        .accounts(reward_accounts::ClaimReward {
            server_admin: admin_pub_key,
            system_program: anchor_lang::solana_program::system_program::id(),
            reward: reward_pda,
            quest_pda: quest_pda,
            quest_program: quest::id(),
        })
        .args(reward_instruction::ClaimReward {
            quest_id,
            user_id,
            server_id,
            provider_id,
        })
        .send()?;

    println!("UpdateReward tx signature: {}", tx_update);

    // --- fetch reward after update ---
    let reward_after_update: Reward = reward.account(reward_pda)?;
    println!(
        "Reward (after manual update) - quest_id: {}, amount: {}",
        reward_after_update.quest_id, reward_after_update.amount
    );

    Ok(())
}
