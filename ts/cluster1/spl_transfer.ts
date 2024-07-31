import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "../wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const token_decimals = 1_000_000n;
// Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K");

// Recipient address
const to = new PublicKey("BApYKNe2yv6u8Wk8uwJwMTPuy5Jw8eQEc2wVYn5gqfFP");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );
        // Transfer the new token to the "toTokenAccount" we just created
        const signature = await transfer(
            connection,
            keypair,
            fromTokenAccount.address,
            toTokenAccount.address,
            keypair.publicKey,
            1 * Number(token_decimals)
        );
        console.log(`Your transfer txid: ${signature}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();

//transfer of token from one account to another
//https://explorer.solana.com/tx/2XACiGzRZUTLcdNsLJvuUGLansV4PfL7RkegkmbiPN8Jmqpg8wUtZ6aB5monnxbnPTrpm5NL5gkarDzw5dQNj9MR?cluster=devnet