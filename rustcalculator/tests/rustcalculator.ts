import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Rustcalculator } from '../target/types/rustcalculator';
import * as assert from "assert";

describe('rustcalculator', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Rustcalculator as Program<Rustcalculator>;

  const calculator = anchor.web3.Keypair.generate();

  it('Create a calculator', async () => {
    // Add your test here.

    const tx = await program.rpc.create('Lets do maths',{
      accounts: {
        calculator: calculator.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [calculator],
    });
    console.log("Your transaction signature", tx);
    const account = await program.account.calculator.fetch(calculator.publicKey);
    //console.log(account.greeting)
    assert.ok(account.greeting === "Lets do maths");
  });

  it("Adds two numbers", async function() {
    
    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)));
    assert.ok(account.greeting === "Lets do maths");
  });

  it('Multiplies two numbers', async function() {

    await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(6)));
    assert.ok(account.greeting === "Lets do maths");
  })

  it('Divides two numbers', async function() {

    await program.rpc.divide(new anchor.BN(10), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(3)));
    assert.ok(account.remainder.eq(new anchor.BN(1)));
    assert.ok(account.greeting === "Lets do maths");
  });
});
