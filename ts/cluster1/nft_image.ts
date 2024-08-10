//first step
import wallet from "../wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Load image
        const imageBuffer = await readFile("../ts/assets/generug.png");

        // Convert image to generic file
        const umiImageFile = createGenericFile(imageBuffer, "generug.png", {
            tags: [{ name: "Content-Type", value: "image/png" }],
          });

        // Upload image
        const [myUri] = await umi.uploader.upload([umiImageFile]);

        console.log("Your image URI: ", myUri);
        //Your image URI:  https://arweave.net/cnuUynn5tQ0lsn_Mdf73kYl8sbh_E56VgJuE9D__3XY
    } catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
