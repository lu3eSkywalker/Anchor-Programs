import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import type { CounterProgram } from "../target/types/counter_program";
describe("Counter Program", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CounterProgram as anchor.Program<CounterProgram>;
  
  const counterKp = new web3.Keypair();

  it("initializes the counter", async () => {
    const txHash = await program.methods
      .init()
      .accounts({
        counter: counterKp.publicKey,
        user: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([counterKp])
      .rpc();

    console.log("Init TX:", txHash);

    const counterAccount = await program.account.counterState.fetch(
      counterKp.publicKey
    );

    assert.equal(counterAccount.count, 1);
    assert.equal(
      counterAccount.authority.toBase58(),
      program.provider.publicKey.toBase58()
    );
  });

  it("doubles the counter", async () => {
    await program.methods
      .double()
      .accounts({
        counter: counterKp.publicKey,
        authority: program.provider.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counterState.fetch(
      counterKp.publicKey
    );
    assert.equal(counterAccount.count, 2);
  });

  it("halves the counter", async () => {
    await program.methods
      .half()
      .accounts({
        counter: counterKp.publicKey,
        authority: program.provider.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counterState.fetch(
      counterKp.publicKey
    );
    assert.equal(counterAccount.count, 1);
  });

  it("adds 10 to the counter", async () => {
    await program.methods
      .add(new BN(10))
      .accounts({
        counter: counterKp.publicKey,
        authority: program.provider.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counterState.fetch(
      counterKp.publicKey
    );
    assert.equal(counterAccount.count, 11);
  });

  it("subtracts 5 from the counter", async () => {
    await program.methods
      .subtract(new BN(5))
      .accounts({
        counter: counterKp.publicKey,
        authority: program.provider.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counterState.fetch(
      counterKp.publicKey
    );
    assert.equal(counterAccount.count, 6);
  });
});
