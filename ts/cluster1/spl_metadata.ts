import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import bs58 from "bs58";

// Define our Mint address
const mint = publicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer
         }

         let data: DataV2Args = {
            name: "My NFT",
            symbol: "NFT",
            uri: "", //empty string since we dont have a URI
            sellerFeeBasisPoints: 0, //fees for royalties
            creators: null, //creators of the NFT
            collection: null, //collection of the NFT
            uses: null, //uses of the NFT
         }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true,
            collectionDetails: null, //collection details
        }

         let tx = createMetadataAccountV3(
             umi,
             {
                 ...accounts,
                 ...args
             }
         )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// Token: https://explorer.solana.com/address/5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K?cluster=devnet
// Metadata: https://explorer.solana.com/address/5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K/metadata?cluster=devnet
