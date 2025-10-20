import { PublicKey } from "@solana/web3.js";
import { Program, AnchorProvider, Wallet, Idl } from "@coral-xyz/anchor";
import achieveGameIdl from "../idl/achieve_game.json";
import rewardAchieIdl from "../idl/reward_achie.json";
import {
  connection,
  ACHIEVE_GAME_PROGRAM_ID,
  REWARD_ACHIE_PROGRAM_ID,
} from "./connection";

// Create Anchor program instances
export const getAchieveGameProgram = (wallet: Wallet) => {
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  return new Program(
    achieveGameIdl as unknown as Idl,
    new PublicKey(ACHIEVE_GAME_PROGRAM_ID),
    provider,
  );
};

export const getRewardAchieProgram = (wallet: Wallet) => {
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  return new Program(
    rewardAchieIdl as unknown as Idl,
    new PublicKey(REWARD_ACHIE_PROGRAM_ID),
    provider,
  );
};

// Derive PDA for game account
export const getGamePDA = (
  admin: PublicKey,
  gameId: number,
  serverId: number,
  providerId: number,
  eventId: number,
) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("game"),
      admin.toBuffer(),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(gameId)]).buffer)),
      Buffer.from(
        new Uint8Array(new BigUint64Array([BigInt(serverId)]).buffer),
      ),
      Buffer.from(
        new Uint8Array(new BigUint64Array([BigInt(providerId)]).buffer),
      ),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(eventId)]).buffer)),
    ],
    new PublicKey(ACHIEVE_GAME_PROGRAM_ID),
  );
};

// Derive PDA for reward account
export const getRewardPDA = (admin: PublicKey, eventId: number) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("reward"),
      admin.toBuffer(),
      Buffer.from(new Uint8Array(new BigUint64Array([BigInt(eventId)]).buffer)),
    ],
    new PublicKey(REWARD_ACHIE_PROGRAM_ID),
  );
};
