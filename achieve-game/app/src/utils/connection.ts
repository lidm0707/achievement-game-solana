import { Connection, clusterApiUrl } from '@solana/web3.js';

// Use devnet by default, can be changed to mainnet-beta for production
export const NETWORK = 'devnet';

export const connection = new Connection(
  clusterApiUrl(NETWORK),
  'confirmed'
);

// Program IDs from Anchor.toml
export const ACHIEVE_GAME_PROGRAM_ID = 'EMuQqLFUEEJouvbMsGuUfkWhT5XxvNCYw1KpwQWc1vUK';
export const REWARD_ACHIE_PROGRAM_ID = 'Ft938dnF35nS6mHvYeP3is3yPV3PLUNB5jyveAWT1HTi';

// Admin public key from lib.rs
export const ADMIN_PUBLIC_KEY = 'AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9';
