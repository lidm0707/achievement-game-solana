# Achievement Game Frontend

A React TypeScript frontend for interacting with the Achievement Game Solana program using Phantom wallet integration.

## Features

- **Phantom Wallet Integration**: Connect and interact with Solana blockchain using Phantom wallet
- **Admin Functions**: Initialize games, update scores, and claim rewards
- **Real-time Status**: View game progress and achievement status
- **Responsive Design**: Works on desktop and mobile devices
- **TypeScript Support**: Full type safety with TypeScript

## Prerequisites

- Node.js 16+ installed
- Phantom wallet browser extension installed
- Solana program already deployed to devnet

## Installation

1. Navigate to the app directory:
```bash
cd achieve-game/app
```

2. Install dependencies:
```bash
npm install
```

## Configuration

The frontend is configured to work with devnet by default. The program IDs are set in `src/utils/connection.ts`:

```typescript
export const ACHIEVE_GAME_PROGRAM_ID = 'EMuQqLFUEEJouvbMsGuUfkWhT5XxvNCYw1KpwQWc1vUK';
export const REWARD_ACHIE_PROGRAM_ID = 'Ft938dnF35nS6mHvYeP3is3yPV3PLUNB5jyveAWT1HTi';
export const ADMIN_PUBLIC_KEY = 'AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9';
```

## Usage

1. Start the development server:
```bash
npm start
```

2. Open http://localhost:3000 in your browser

3. Connect your Phantom wallet

4. If you're the admin (AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9), you can:
   - Initialize a new game with parameters
   - Update game scores
   - Claim rewards when max score is reached

## Program Functions

### Initialize Game
Creates a new game instance with the following parameters:
- Game ID
- Server ID  
- Provider ID
- Event ID
- Deadline (Unix timestamp)
- Maximum score

### Update Score
Increments the current game score by 1 (admin only)

### Claim Reward
Claims the reward when the game reaches maximum score (admin only)

## Project Structure

```
src/
├── components/
│   └── GameInterface.tsx    # Main game interaction component
├── utils/
│   ├── connection.ts        # Solana connection and constants
│   └── program.ts          # Anchor program utilities
├── idl/
│   ├── achieve_game.json   # IDL for achieve_game program
│   └── reward_achie.json   # IDL for reward_achie program
├── App.tsx                 # Main app component with wallet providers
├── index.tsx              # Entry point
└── styles/
    ├── App.css
    └── index.css
```

## Dependencies

- React 18
- TypeScript
- Solana Web3.js
- Solana Wallet Adapter
- Anchor (@coral-xyz/anchor)
- React Scripts

## Network Configuration

The app is configured for **devnet** by default. To change networks:

1. Update `NETWORK` in `src/utils/connection.ts`
2. Update program IDs if deploying to a different network

## Security Notes

- Only the admin wallet can initialize games, update scores, and claim rewards
- The admin public key is hardcoded and should match your deployed program
- Always verify you're connected to the correct network before making transactions

## Troubleshooting

1. **Wallet not connecting**: Ensure Phantom wallet is installed and unlocked
2. **Transaction failing**: Check if you have enough SOL for gas fees
3. **Admin functions not working**: Verify your wallet address matches the admin public key
4. **Program account not found**: Ensure the program is deployed to the correct network

## Development

To build for production:
```bash
npm run build
```

To run tests:
```bash
npm test
```
