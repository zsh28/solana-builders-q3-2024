# Introduction

This repository showcases work done for the WBA Turbine 2024 Q3 Cohort.

## Turbine3 Work

### Class Index
- [30/07/2024 - Class 1](#30072024---class-1)
- [31/07/2024 - Class 2](#31072024---class-2)
- [01/08/2024 - Class 3](#01082024---class-3)

---

## 30/07/2024 - Class 1

In Class 1, we covered the process of creating and managing tokens on the Solana blockchain using the Solana Program Library (SPL). The main objectives included:

1. **Creating a Token (Mint Account)**:
   - A mint account represents a new token type. We initialized this using the `createMint` function from the SPL Token library. This mint account is used to create tokens that can be distributed to other accounts.

2. **Creating an Associated Token Account (ATA)**:
   - An ATA is a token account associated with a specific wallet. It holds tokens linked to that wallet's address. We created an ATA using the `getOrCreateAssociatedTokenAccount` function.

3. **Minting Tokens to the ATA**:
   - We minted tokens into the ATA using the `mintTo` function. This involves specifying the mint account, the ATA, and the amount of tokens to mint.

### Files for this task are located in:

    ts\cluster1\spl_init.ts
    ts\cluster1\spl_mint.ts

### Code Snippets

**File: `spl_init.ts`**

```typescript
import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6
    );
    console.log(`Successfully created mint: ${mint}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
```

**File: `spl_mint.ts`**

```typescript
import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "../wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;
const mint = new PublicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K");

(async () => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Your ATA is: ${ata.address.toBase58()}`);

    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      BigInt(100) * token_decimals
    );
    console.log(`Your mint txid: ${mintTx}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
```

You can view the transaction details for the mint operation [here](https://explorer.solana.com/tx/5t9ATaWSw8gnR6BFFNoCM5ZjxHay2JrN9FNjRea6CfkXrQRGwkpu69YhaDVgjqitGMLhwbQS5yTXNi1V4y8GjxNr?cluster=devnet).

---

## 31/07/2024 - Class 2

In Class 2, we expanded our understanding of the Solana blockchain by adding metadata to tokens and transferring tokens between accounts. Additionally, we introduced the use of the Umi library, which simplifies interactions with Solana and Metaplex.

### Key Concepts Covered

1. **Using Umi for Solana Interactions**:
   - Umi is a toolkit that simplifies creating and managing Solana accounts, transactions, and interacting with various Solana programs. We used Umi to create connections, handle keypairs, and manage identities securely.

2. **Creating Metadata for the Token**:
   - Using the Metaplex Token Metadata program and Umi, we associated metadata with our token. This metadata includes the token's name, symbol, URI, and other properties, which are crucial for NFTs and other tokenized assets. For more details, refer to the [Metaplex Token Metadata Documentation](https://developers.metaplex.com/token-metadata/token-standard).

3. **Transferring Tokens**:
   - We demonstrated how to transfer tokens between accounts, including setting up the required associated token accounts (ATA) for both the sender and recipient.

### Files for this task are located in:

    ts\cluster1\spl_metadata.ts
    ts\cluster1\spl_transfer.ts

### Code Snippets

**File: `spl_metadata.ts`**

```typescript
import wallet from "../wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import bs58 from "bs58";

// Define the Mint address
const mint = publicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K");

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
            uri: "", // Empty URI for now
            sellerFeeBasisPoints: 0,
            creators: null,
            collection: null,
            uses: null,
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true,
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        );

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
```

**File: `spl_transfer.ts`**

```typescript
import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "../wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const token_decimals = 1_000_000n;
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const mint = new PublicKey("5N8rqmmhP8bwLWpHpCqc1BRHdWCwHgjKEzAhgke5UR6K");
const to = new PublicKey("BApYKNe2yv6u8Wk8uwJwMTPuy5Jw8eQEc2wVYn5gqfFP");

(async () => {
    try {
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );

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
```

These examples highlight how Umi simplifies Solana development by providing tools for easy account management, transaction handling, and program interaction. You can view the transaction details for the token transfer [here](https://explorer.solana.com/tx/2XACiGzRZUTLcdNsLJvuUGLansV4PfL7RkegkmbiPN8Jmqpg8wUtZ6aB5monnxbnPTrpm5NL5gkarDzw5dQNj9MR?cluster=devnet).

---

## 

01/08/2024 - Class 3

In Class 3, we delved into the process of creating and minting NFTs (Non-Fungible Tokens) on the Solana blockchain using the Metaplex framework and Umi library.

### Key Concepts Covered

1. **Uploading NFT Images**:
   - We used Umi and the Irys uploader to store images on Arweave, a decentralized storage network. This involved converting image files into a format suitable for uploading and then storing them on Arweave to get a URI.

2. **Creating NFT Metadata**:
   - The metadata for the NFT, including its name, symbol, description, and image URI, was created and uploaded to Arweave. This metadata is essential for defining the NFT's attributes and linking it to the associated image.

3. **Minting the NFT**:
   - We minted the NFT by associating the previously created metadata with a new mint address on the Solana blockchain. This process involves creating a unique mint address, setting the NFT's metadata URI, and finalizing the transaction on the blockchain.

### Files for this task are located in:

    ts\cluster3\nft_image.ts
    ts\cluster3\nft_metadata.ts
    ts\cluster3\nft_mint.ts

### Code Snippets

**File: `nft_image.ts`**

```typescript
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
```

**File: `nft_metadata.ts`**

```typescript
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
```

**File: `nft_mint.ts`**

```typescript
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
```
### Class 3 Additional Information

**Transaction Details:** [View Transaction](https://explorer.solana.com/tx/27m1CxDMSJ8JTAkyXR9eQ6HMz66gyNkZUEyziGQ2A4b6uGdPNr9WJ16yT7TQJDWpsvNZwXWFYoEz86bjs3qUVpJ3?cluster=devnet)

**Mint Address:** [5p4RGJ7Z1VNorHr18jqczJwJ9RbDr4rahfrcAgToehMz](https://explorer.solana.com/address/5p4RGJ7Z1VNorHr18jqczJwJ9RbDr4rahfrcAgToehMz?cluster=devnet)

**NFT Details:** [View NFT on Solana Explorer](https://explorer.solana.com/address/BH4xprfV6pf6R64HQ1fNqW2DvebL8gCtV2Sb2QDdBf5X/instructions?cluster=devnet)

---