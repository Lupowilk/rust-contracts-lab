const anchor = require("@coral-xyz/anchor");
const { Connection, Keypair, PublicKey } = require("@solana/web3.js");
const { getOrCreateAssociatedTokenAccount } = require("@solana/spl-token");
const fs = require("fs");
const path = require("path");

async function main() {
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const walletPath = path.join(process.env.HOME, ".config/solana/id.json");
  const walletKeypair = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(walletPath, "utf-8")))
  );
  const wallet = new anchor.Wallet(walletKeypair);

  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  const idlPath = path.join(__dirname, "../target/idl/token_contract.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
  const programId = new PublicKey("7XF9upHxdV5iVWehmVaAsPzzDwyrozdtgpEFFHbBtDTj");
  const program = new anchor.Program(idl, provider);

  const mintKeypair = Keypair.generate();
  console.log("Mint address:", mintKeypair.publicKey.toBase58());

  console.log("Creating mint...");
  await program.methods
    .createMint(9)
    .accounts({
      mint: mintKeypair.publicKey,
      payer: walletKeypair.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([mintKeypair])
    .rpc();
  console.log("Mint created!");

  console.log("Creating token account...");
  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    walletKeypair,
    mintKeypair.publicKey,
    walletKeypair.publicKey
  );
  console.log("Token account:", tokenAccount.address.toBase58());

  console.log("Minting tokens...");
  await program.methods
    .mintTokens(new anchor.BN(1000000000))
    .accounts({
      mint: mintKeypair.publicKey,
      receiverAccount: tokenAccount.address,
      payer: walletKeypair.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    })
    .rpc();
  console.log("Minted 1 token successfully!");
}

main().catch(console.error);
