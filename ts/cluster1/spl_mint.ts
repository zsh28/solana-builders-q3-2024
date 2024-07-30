import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K");

(async () => {
  try {
    // Create an ATA
    //why creata a ata? - An associated token account (ATA) is a token account that is associated with a particular wallet address.
    // It is used to hold tokens that are associated with a particular wallet address.
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);

    // Mint to ATA
    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      BigInt(100) * token_decimals
    );
    console.log(`Your mint txid: ${mintTx}`);
    {
      /*
        Your ata is: CBP2JETg7E2mbg17b4bW34ohm3jXsHPP3psNeux93Q4k
        Your mint txid: 5t9ATaWSw8gnR6BFFNoCM5ZjxHay2JrN9FNjRea6CfkXrQRGwkpu69YhaDVgjqitGMLhwbQS5yTXNi1V4y8GjxNr
    */
    }
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
