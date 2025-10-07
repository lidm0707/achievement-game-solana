# Achievement Game Solana

This project implements an achievement game on the Solana blockchain.

## Getting Started

### Prerequisites

*   [Node.js](https://nodejs.org/en/download/)
*   [Rust](https://www.rust-lang.org/tools/install)
*   [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)
*   [Anchor](https://www.anchor-lang.com/docs/installation)
*   [Surfpool](https://surfpool.run)

### Installation

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/achievement-game-solana.git
    cd achievement-game-solana
    ```

2.  **Install project dependencies:**

    ```bash
    npm install
    ```

3.  **Install Surfpool:**

    Follow the instructions on the [Surfpool website](https://surfpool.run) to install it on your system.

### Building the Program

Build the Anchor program from within the `achieve-game` directory:

```bash
cd achieve-game
anchor build
```

After building, you will get a new key-pair for the program. You can find it at `target/deploy/<program-name>-keypair.json`.

### Deployment and Testing

This project uses [Surfpool](https://surfpool.run) for local development and testing.

1.  **Start Surfpool:**

    In the root directory of the project, start Surfpool:

    ```bash
    surfpool start
    ```

2.  **Configure your wallet:**

    When you build the project for the first time, a new program keypair is generated. You need to make sure your local wallet has enough SOL to deploy and interact with the program.

    -   Get your wallet address:
        ```bash
        solana address
        ```
    -   Airdrop some SOL to your wallet:
        ```bash
        solana airdrop 2
        ```

3.  **Run the client application:**

    The `call-program` directory contains a client application to interact with the Solana program.

    -   Navigate to the `call-program` directory:
        ```bash
        cd ../call-program
        ```
    -   Run the client:
        ```bash
        cargo run
        ```
