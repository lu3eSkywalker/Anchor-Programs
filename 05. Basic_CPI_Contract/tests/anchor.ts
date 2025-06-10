import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { BasicCpiContract } from "../target/types/basic_cpi_contract";

describe("cpi-contract", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CpiContract as anchor.Program<BasicCpiContract>;
  
  it("Transfers SOL from wallet to recipient", async () => {
    // Generate keypair for the recipient
    const recipient = new web3.Keypair();

    // Airdrop some SOL to the sender to ensure it has enough (in playground, pg.wallet is already funded)
    const amount = new BN(1_000_000_000); // 1 SOL

    // Send the transaction
    const txHash = await program.methods
      .solTransfer(amount)
      .accounts({
        sender: program.provider.publicKey,
        recipient: recipient.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);

    // Fetch the recipient's account info to check balance
    const accountInfo = await program.provider.connection.getAccountInfo(recipient.publicKey);
    console.log("Recipient lamports:", accountInfo?.lamports);

    // Validate that recipient received 1 SOL
    assert.equal(accountInfo?.lamports, 1_000_000_000);
  });
});