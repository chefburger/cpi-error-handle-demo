import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiErrorHandle } from "../target/types/cpi_error_handle";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { createMint, createAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("cpi-error-handle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.cpiErrorHandle as Program<CpiErrorHandle>;
  const provider = anchor.getProvider();

  it("test cpi error handling for system cpi", async () => {
    // Create a new keypair for the mint
    const mintKeypair = Keypair.generate();

    const payerKeypair = Keypair.generate();
    const recipientKeypair = Keypair.generate();

    console.log("Created mint:", mintKeypair.publicKey.toString());
    console.log("Created user:", payerKeypair.publicKey.toString());

    // Create the token mint
    const mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      payerKeypair.publicKey,
      payerKeypair.publicKey,
      9,
      mintKeypair
    );

    console.log("Token mint created:", mint.toString());

    // from ATA
    const fromTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      payerKeypair.publicKey
    );

    console.log("From token account created:", fromTokenAccount.toString());

    // to ATA
    const toTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      recipientKeypair.publicKey
    );

    console.log("To token account created:", toTokenAccount.toString());

    // Mint 1 token to the from account
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mint,
      fromTokenAccount,
      payerKeypair,
      1
    );

    // Test the CPI error handling
    try {
      await program.methods
        .testSystemCpi()
        .accounts({
          user: payerKeypair.publicKey,
          fromTokenAccount: fromTokenAccount,
          toTokenAccount: toTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([provider.wallet.payer, payerKeypair])
        .rpc();
    } catch (error) {
      console.log("Expected error (insufficient tokens):", error);
    }
  });

  it("test cpi error handling for custom program", async () => {
    const userKeypair = Keypair.generate();

    console.log("Created user for custom CPI test:", userKeypair.publicKey.toString());

    // Test the custom CPI error handling
    try {
      await program.methods
        .testCustomCpi()
        .accounts({
          user: userKeypair.publicKey,
          simpleErrorProgram: anchor.workspace.simpleError.programId,
        })
        .signers([provider.wallet.payer, userKeypair])
        .rpc();
    } catch (error) {
      console.log("Expected error from custom program:", error);
    }
  });
});
