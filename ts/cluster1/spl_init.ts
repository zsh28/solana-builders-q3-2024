import { Keypair, Connection, Commitment } from "@solana/web3.js";
//commitment - The level of commitment desired when querying state
import { createMint } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    // Start here
    //
    {
      /*
        How to Create a Token
        Creating tokens is done by creating what is called a "mint account".
        This mint account is later used to mint tokens to a user's token account.
     */
    }
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6
    );
    console.log(`successfully created mint: ${mint}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

// mint_address = "5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K"
{
  /*
// Mint(
    // Optional authority used to mint new tokens. The mint authority may only
    be provided during mint creation. If no authority is present,
    then the mint has a fixed supply and no further tokens may be minted.
    pub mint_authority: Option<Pubkey>,
    // Total supply of tokens
    pub supply: u64,
    // Number of base 10 digits to the right of the decimal place
    pub decimals: u8,
    // Is `true` if this structure has been initialized
    pub is_initialized: bool,
    // Optional authority to freeze the token accounts
    pub freeze_authority: Option<Pubkey>,
)
*/
}
