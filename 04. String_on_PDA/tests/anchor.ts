import * as anchor from "@coral-xyz/anchor";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { StringOnPda } from "../target/types/string_on_pda";

describe("StringOnPda", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StringOnPda as anchor.Program<StringOnPda>;
  
  it("createPdaAccount", async () => {
    const testString = "Hello There";

    // Derive the PDA
    const [pdaAccount, bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("string1"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    // Send transaction
    const txHash = await program.methods
      .createPdaAccount(testString)
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
    const account = await program.account.stringAccount.fetch(pdaAccount);

    console.log("On-chain string account:", {
      owner: account.owner.toBase58(),
      stringOnBlockchain: account.stringOnBlockchain,
    });

    // Assertions
    assert(account.owner.equals(program.provider.publicKey));
    assert.equal(account.stringOnBlockchain, testString);
  });
});
