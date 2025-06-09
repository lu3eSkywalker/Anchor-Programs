import * as anchor from "@coral-xyz/anchor";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { AnchorPda } from "../target/types/anchor_pda";


describe("StakeProgram", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.YourProgram as anchor.Program<AnchorPda>;
  
  it("createPdaAccount", async () => {
    // Derive the PDA
    const [pdaAccount, bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("client1"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    // Send transaction
    const txHash = await program.methods
      .createPdaAccount()
      .accounts({
        payer: program.provider.publicKey,
        pdaAccount: pdaAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);

    // Fetch the PDA account
    const account = await program.account.stakeAccount.fetch(pdaAccount);

    console.log("On-chain stake account:", {
      owner: account.owner.toBase58(),
      stakedAmount: account.stakedAmount.toString(),
      bump: account.bump,
    });

    // Assertions
    assert(account.owner.equals(program.provider.publicKey));
    assert.equal(account.stakedAmount.toNumber(), 0);
    assert.equal(account.bump, bump);
  });
});
