import React, { useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { connection } from "./utils/connection";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { GameInterface } from "./components/GameInterface";
import "./App.css";

// Default styles for the wallet adapter
require("@solana/wallet-adapter-react-ui/styles.css");

function AppContent() {
  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => connection.rpcEndpoint, []);

  // @solana/wallet-adapter-wallets includes all the adapters but supports tree shaking.
  // Only the wallets you configure here will be compiled into your application.
  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
    [],
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="App">
            <header className="App-header">
              <h1>Achievement Game on Solana</h1>
              <div className="wallet-section">
                <WalletMultiButton className="wallet-button" />
              </div>
            </header>
            <main>
              <GameInterface />
            </main>
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

function App() {
  return <AppContent />;
}

export default App;
