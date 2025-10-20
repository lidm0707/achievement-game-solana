import React, { useState, useEffect, useCallback } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { SystemProgram } from "@solana/web3.js";
import {
  getAchieveGameProgram,
  getRewardAchieProgram,
  getGamePDA,
  getRewardPDA,
} from "../utils/program";
import { ADMIN_PUBLIC_KEY } from "../utils/connection";

interface GameState {
  gameId: number;
  serverId: number;
  providerId: number;
  eventId: number;
  deadline: number;
  maxScore: number;
  currentScore: number;
}

export const GameInterface: React.FC = () => {
  const { publicKey, signTransaction, connected } = useWallet();
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [formData, setFormData] = useState({
    gameId: "",
    serverId: "",
    providerId: "",
    eventId: "",
    deadline: "",
    maxScore: "",
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const initializeGame = async () => {
    if (!publicKey || !signTransaction) {
      setMessage("Please connect your wallet first");
      return;
    }

    if (publicKey.toString() !== ADMIN_PUBLIC_KEY) {
      setMessage("Only the admin can initialize games");
      return;
    }

    try {
      setLoading(true);
      setMessage("");

      const program = getAchieveGameProgram({
        publicKey,
        signTransaction,
      } as any);

      const gameId = parseInt(formData.gameId);
      const serverId = parseInt(formData.serverId);
      const providerId = parseInt(formData.providerId);
      const eventId = parseInt(formData.eventId);
      const deadline = parseInt(formData.deadline);
      const maxScore = parseInt(formData.maxScore);

      const [gamePDA] = getGamePDA(
        publicKey,
        gameId,
        serverId,
        providerId,
        eventId,
      );

      await program.methods
        .initialize(gameId, serverId, providerId, deadline, eventId, maxScore)
        .accounts({
          game: gamePDA,
          admin: publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      setMessage("Game initialized successfully!");
      setGameState({
        gameId,
        serverId,
        providerId,
        eventId,
        deadline,
        maxScore,
        currentScore: 0,
      });
    } catch (error) {
      console.error("Error initializing game:", error);
      setMessage(
        `Error: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
    } finally {
      setLoading(false);
    }
  };

  const updateScore = async () => {
    if (!publicKey || !signTransaction) {
      setMessage("Please connect your wallet first");
      return;
    }

    if (publicKey.toString() !== ADMIN_PUBLIC_KEY) {
      setMessage("Only the admin can update scores");
      return;
    }

    if (!gameState) {
      setMessage("Please initialize a game first");
      return;
    }

    try {
      setLoading(true);
      setMessage("");

      const achieveGameProgram = getAchieveGameProgram({
        publicKey,
        signTransaction,
      } as any);
      const rewardAchieProgram = getRewardAchieProgram({
        publicKey,
        signTransaction,
      } as any);

      const [gamePDA] = getGamePDA(
        publicKey,
        gameState.gameId,
        gameState.serverId,
        gameState.providerId,
        gameState.eventId,
      );
      const [rewardPDA] = getRewardPDA(publicKey, gameState.eventId);

      await achieveGameProgram.methods
        .ongoing(gameState.eventId)
        .accounts({
          game: gamePDA,
          admin: publicKey,
          reward: rewardPDA,
          rewardProgram: rewardAchieProgram.programId,
        })
        .rpc();

      setMessage("Score updated successfully!");
      setGameState({
        ...gameState,
        currentScore: gameState.currentScore + 1,
      });
    } catch (error) {
      console.error("Error updating score:", error);
      setMessage(
        `Error: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
    } finally {
      setLoading(false);
    }
  };

  const claimReward = async () => {
    if (!publicKey || !signTransaction) {
      setMessage("Please connect your wallet first");
      return;
    }

    if (publicKey.toString() !== ADMIN_PUBLIC_KEY) {
      setMessage("Only the admin can claim rewards");
      return;
    }

    if (!gameState) {
      setMessage("Please initialize a game first");
      return;
    }

    try {
      setLoading(true);
      setMessage("");

      const achieveGameProgram = getAchieveGameProgram({
        publicKey,
        signTransaction,
      } as any);
      const rewardAchieProgram = getRewardAchieProgram({
        publicKey,
        signTransaction,
      } as any);

      const [gamePDA] = getGamePDA(
        publicKey,
        gameState.gameId,
        gameState.serverId,
        gameState.providerId,
        gameState.eventId,
      );
      const [rewardPDA] = getRewardPDA(publicKey, gameState.eventId);

      await achieveGameProgram.methods
        .claimReward(gameState.eventId)
        .accounts({
          game: gamePDA,
          admin: publicKey,
          reward: rewardPDA,
          rewardProgram: rewardAchieProgram.programId,
        })
        .rpc();

      setMessage("Reward claimed successfully!");
    } catch (error) {
      console.error("Error claiming reward:", error);
      setMessage(
        `Error: ${error instanceof Error ? error.message : "Unknown error"}`,
      );
    } finally {
      setLoading(false);
    }
  };

  const fetchGameState = useCallback(async () => {
    if (!publicKey || !gameState) return;

    try {
      const program = getAchieveGameProgram({
        publicKey,
        signTransaction,
      } as any);
      const [gamePDA] = getGamePDA(
        publicKey,
        gameState.gameId,
        gameState.serverId,
        gameState.providerId,
        gameState.eventId,
      );

      const gameAccount = await program.account.progress.fetch(gamePDA);
      setGameState({
        ...gameState,
        currentScore: Number((gameAccount as any).score),
      });
    } catch (error) {
      console.error("Error fetching game state:", error);
    }
  }, [publicKey, gameState, signTransaction]);

  useEffect(() => {
    if (connected && gameState) {
      fetchGameState();
    }
  }, [connected, gameState, fetchGameState]);

  if (!connected) {
    return (
      <div className="game-interface">
        <h2>Please connect your wallet to continue</h2>
      </div>
    );
  }

  const isAdmin = publicKey?.toString() === ADMIN_PUBLIC_KEY;

  return (
    <div className="game-interface">
      <div className="user-info">
        <p>
          <strong>Connected Wallet:</strong> {publicKey?.toString()}
        </p>
        <p>
          <strong>Role:</strong> {isAdmin ? "Admin" : "User"}
        </p>
      </div>

      {!gameState && isAdmin && (
        <div className="initialize-section">
          <h3>Initialize New Game</h3>
          <div className="form-group">
            <label>Game ID:</label>
            <input
              type="number"
              name="gameId"
              value={formData.gameId}
              onChange={handleInputChange}
              placeholder="Enter game ID"
            />
          </div>
          <div className="form-group">
            <label>Server ID:</label>
            <input
              type="number"
              name="serverId"
              value={formData.serverId}
              onChange={handleInputChange}
              placeholder="Enter server ID"
            />
          </div>
          <div className="form-group">
            <label>Provider ID:</label>
            <input
              type="number"
              name="providerId"
              value={formData.providerId}
              onChange={handleInputChange}
              placeholder="Enter provider ID"
            />
          </div>
          <div className="form-group">
            <label>Event ID:</label>
            <input
              type="number"
              name="eventId"
              value={formData.eventId}
              onChange={handleInputChange}
              placeholder="Enter event ID"
            />
          </div>
          <div className="form-group">
            <label>Deadline (Unix Timestamp):</label>
            <input
              type="number"
              name="deadline"
              value={formData.deadline}
              onChange={handleInputChange}
              placeholder="Enter deadline"
            />
          </div>
          <div className="form-group">
            <label>Max Score:</label>
            <input
              type="number"
              name="maxScore"
              value={formData.maxScore}
              onChange={handleInputChange}
              placeholder="Enter max score"
            />
          </div>
          <button
            onClick={initializeGame}
            disabled={loading}
            className="btn btn-primary"
          >
            {loading ? "Processing..." : "Initialize Game"}
          </button>
        </div>
      )}

      {gameState && (
        <div className="game-status">
          <h3>Game Status</h3>
          <div className="status-grid">
            <div>
              <strong>Game ID:</strong> {gameState.gameId}
            </div>
            <div>
              <strong>Server ID:</strong> {gameState.serverId}
            </div>
            <div>
              <strong>Provider ID:</strong> {gameState.providerId}
            </div>
            <div>
              <strong>Event ID:</strong> {gameState.eventId}
            </div>
            <div>
              <strong>Current Score:</strong> {gameState.currentScore}
            </div>
            <div>
              <strong>Max Score:</strong> {gameState.maxScore}
            </div>
            <div>
              <strong>Progress:</strong>{" "}
              {((gameState.currentScore / gameState.maxScore) * 100).toFixed(1)}
              %
            </div>
          </div>

          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{
                width: `${(gameState.currentScore / gameState.maxScore) * 100}%`,
              }}
            />
          </div>

          {isAdmin && (
            <div className="admin-actions">
              <h4>Admin Actions</h4>
              <button
                onClick={updateScore}
                disabled={
                  loading || gameState.currentScore >= gameState.maxScore
                }
                className="btn btn-secondary"
              >
                {loading ? "Processing..." : "Update Score (+1)"}
              </button>
              <button
                onClick={claimReward}
                disabled={
                  loading || gameState.currentScore < gameState.maxScore
                }
                className="btn btn-success"
              >
                {loading ? "Processing..." : "Claim Reward"}
              </button>
              <button onClick={fetchGameState} className="btn btn-info">
                Refresh Status
              </button>
            </div>
          )}
        </div>
      )}

      {message && (
        <div
          className={`message ${message.includes("Error") ? "error" : "success"}`}
        >
          {message}
        </div>
      )}
    </div>
  );
};
