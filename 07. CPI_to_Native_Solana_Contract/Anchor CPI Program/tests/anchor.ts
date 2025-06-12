import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { CpiContract } from "../target/types/cpi_contract";
describe("cpi-calculator", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CpiContract as anchor.Program<CpiContract>;
  
  const counter = web3.Keypair.generate();

  const COUNTER_PROGRAM_ID = new web3.PublicKey(
    "NsXgNXLRq8RUYvqfk8YCWL9WAzAWg2ZEGoWekE6qyZx"
  );

  it("Initializes the counter account", async () => {
    const txHash = await program.methods
      .initialize()
      .accounts({
        dataAccount: counter.publicKey,
        userAccount: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
        cpiProgram: COUNTER_PROGRAM_ID,
      })
      .signers([counter])
      .preInstructions([
        web3.SystemProgram.createAccount({
          fromPubkey: program.provider.publicKey,
          newAccountPubkey: counter.publicKey,
          lamports: await program.provider.connection.getMinimumBalanceForRentExemption(4),
          space: 4,
          programId: COUNTER_PROGRAM_ID,
        }),
      ])
      .rpc();

    console.log(`✅ Init tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);

    const acct = await program.provider.connection.getAccountInfo(counter.publicKey);
    const counterValue = new BN(acct!.data, "le").toNumber();
    console.log("Counter initialized to:", counterValue);
    assert.equal(counterValue, 1);
  });

  it("Doubles the counter value", async () => {
    const txHash = await program.methods
      .double()
      .accounts({
        dataAccount: counter.publicKey,
        userAccount: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
        cpiProgram: COUNTER_PROGRAM_ID,
      })
      .rpc();

    console.log(`✅ Double tx: ${txHash}`);
    await program.provider.connection.confirmTransaction(txHash);

    const acct = await program.provider.connection.getAccountInfo(counter.publicKey);
    const counterValue = new BN(acct!.data, "le").toNumber();
    console.log("Counter doubled to:", counterValue);
    assert.equal(counterValue, 2);
  });
});
