
import React, { useState } from "react";
import { Connection, PublicKey, Transaction, SystemProgram } from "@solana/web3.js";

const App = () => {
  const connection = new Connection("https://api.devnet.solana.com");
  const [wallet, setWallet] = useState(null);
  const [freelancer, setFreelancer] = useState("");
  const [amount, setAmount] = useState(0);

  const initializeEscrow = async () => {
    if (!wallet || !freelancer || !amount) {
      alert("Please provide all required details.");
      return;
    }

    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: wallet.publicKey,
        toPubkey: new PublicKey(freelancer),
        lamports: amount * 10 ** 9, // Convert SOL to lamports
      })
    );

    try {
      const signature = await connection.sendTransaction(transaction, [wallet]);
      console.log("Transaction Successful:", signature);
    } catch (err) {
      console.error("Transaction Failed:", err);
    }
  };

  return (
    <div>
      <h1>Flow Platform</h1>
      <input
        type="text"
        placeholder="Freelancer Wallet Address"
        onChange={(e) => setFreelancer(e.target.value)}
      />
      <input
        type="number"
        placeholder="Amount (SOL)"
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={initializeEscrow}>Initialize Escrow</button>
    </div>
  );
};

export default App;
