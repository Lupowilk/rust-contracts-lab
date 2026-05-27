const anchor = require("@coral-xyz/anchor");
const { Connection, Keypair, PublicKey } = require("@solana/web3.js");
const { getAssociatedTokenAddressSync } = require("@solana/spl-token");
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

  const idlPath = path.join(__dirname, "../target/idl/nft_contract.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
  const program = new anchor.Program(idl, provider);

  const mintKeypair = Keypair.generate();
  console.log("Mint address:", mintKeypair.publicKey.toBase58());

  const [metadataPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  );

  const tokenAccountAddress = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    walletKeypair.publicKey
  );

  console.log("Minting NFT...");
  await program.methods
    .mintNft(
      "My First NFT",
      "MNFT",
      "https://arweave.net/placeholder"
    )
    .accounts({
      payer: walletKeypair.publicKey,
      mint: mintKeypair.publicKey,
      tokenAccount: tokenAccountAddress,
      metadata: metadataPda,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
    })
    .signers([mintKeypair])
    .rpc();

  console.log("NFT minted!");
  console.log("Mint:", mintKeypair.publicKey.toBase58());
  console.log("Metadata:", metadataPda.toBase58());
  console.log("Token account:", tokenAccountAddress.toBase58());
}

main().catch(console.error);
