//2nd step
import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://arweave.net/cnuUynn5tQ0lsn_Mdf73kYl8sbh_E56VgJuE9D__3XY";
     const metadata = {
             name: "zsh28",
             symbol: "zsh",
             description: "zsh28 nft for wba cohort q3",
             image: image,
             attributes: [
                {trait_type: 'trait', value: 'value'},
                {trait_type: 'trait', value: 'value'},
                {trait_type: 'trait', value: 'value'},
            ],
             properties: {
                 files: [
                    {
                         type: "image/png",
                         uri: image,
                     },
                 ]
        },
        creators: [
            {
                address: keypair.publicKey,
                share: 100
            }
        ]
         };
         const myUri = await umi.uploader.uploadJson(metadata);
         console.log("Your json URI: ", myUri);
         //Your json URI:  https://arweave.net/pH69ELMW2S7bGfhJC09dUMkfHYSZFRqILC8rWurOSzw
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
