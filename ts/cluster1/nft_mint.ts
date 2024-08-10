//step 3
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = await createNft(
        umi,
        {
            mint: generateSigner(umi),
            name: "zsh28 NFT",
            symbol: "zsh",
            uri: "https://arweave.net/pH69ELMW2S7bGfhJC09dUMkfHYSZFRqILC8rWurOSzw",
            sellerFeeBasisPoints: percentAmount(0, 2),
        }
    )
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

//transaction: https://explorer.solana.com/tx/27m1CxDMSJ8JTAkyXR9eQ6HMz66gyNkZUEyziGQ2A4b6uGdPNr9WJ16yT7TQJDWpsvNZwXWFYoEz86bjs3qUVpJ3?cluster=devnet
//Mint Address:  5p4RGJ7Z1VNorHr18jqczJwJ9RbDr4rahfrcAgToehMz
//nft: https://explorer.solana.com/address/BH4xprfV6pf6R64HQ1fNqW2DvebL8gCtV2Sb2QDdBf5X/instructions?cluster=devnet