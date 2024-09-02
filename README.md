# Introduction

This repository showcases work done for the WBA Turbine 2024 Q3 Cohort.

## Turbine3 Work

### Class Index
- [30/07/2024 - Class 1](#30072024---class-1)
- [31/07/2024 - Class 2](#31072024---class-2)
- [01/08/2024 - Class 3](#01082024---class-3)
- [06/08/2024 - Class 4](#06082024---class-4)
- [07/08/2024 - Class 5](#07082024---class-5)
- [08/08/2024 - Class 6](#08082024---class-6)
- [13/08/2024 - 15/08/2024 - Classes 7 - 9](#13082024---15082024---classes-7---9)
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
---

## 01/08/2024 - Class 3

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

### Class 3 Additional Information

**Transaction Details:** [View Transaction](https://explorer.solana.com/tx/27m1CxDMSJ8JTAkyXR9eQ6HMz66gyNkZUEyziGQ2A4b6uGdPNr9WJ16yT7TQJDWpsvNZwXWFYoEz86bjs3qUVpJ3?cluster=devnet)

**Mint Address:** [5p4RGJ7Z1VNorHr18jqczJwJ9RbDr4rahfrcAgToehMz](https://explorer.solana.com/address/5p4RGJ7Z1VNorHr18jqczJwJ9RbDr4rahfrcAgToehMz?cluster=devnet)

**NFT Details:** [View NFT on Solana Explorer](https://explorer.solana.com/address/BH4xprfV6pf6R64HQ1fNqW2DvebL8gCtV2Sb2QDdBf5X/instructions?cluster=devnet)

---

## 06/08/2024 - Class 4

In Class 4, we explored the implementation of a simple vault using Anchor, a framework for Solana smart contracts. The vault allows users to deposit and withdraw SOL, with additional functionality to close the vault and transfer the remaining balance back to the user.

### Key Concepts Covered

1. **Anchor Vault Program**:
   - We developed a vault program using Anchor. The program includes functionality to initialize the vault, deposit SOL into it, withdraw SOL from it, and close the vault, returning all remaining funds to the user.

2. **Understanding Context and Accounts**:
   - The Anchor framework simplifies the process of managing accounts and interactions within Solana programs by using Rust macros and decorators. This class focused on how to structure the context and account data for managing vault operations.

3. **Program Structure and Functions**:
   - The vault program consists of several core functions: `initialize`, `deposit`, `withdraw`, and `close`. Each function interacts with the vault state and system program to manage funds securely.

### Source Files/code 
- This can be found in /anchor-vault 

---

## 07/08/2024 - Class 5

In Class 5, we explored the implementation of an escrow program on Solana using Anchor, a framework for building secure and maintainable smart contracts. The class provided a detailed walkthrough of setting up and managing an escrow service where assets are held in trust until specific conditions are met.

### Key Concepts Covered

1. **Anchor-Escrow Program Overview**:
   - The session focused on developing an escrow program where a depositor can lock funds, and a recipient can claim them once predetermined conditions are satisfied. This concept is crucial for scenarios like trustless transactions or conditional payments.

2. **Escrow Initialization**:
   - We initialized the escrow account, specifying the conditions under which the locked assets can be released. This involved creating and configuring the necessary program accounts and setting up the terms of the escrow.

3. **Depositing and Releasing Funds**:
   - The class demonstrated how to deposit funds into the escrow and how those funds are released when the conditions are met. We also discussed how to handle the scenarios where the escrow might need to be canceled or funds returned to the depositor.

4. **Security and Validation**:
   - Ensuring the security of the escrow process was a significant part of the session. We covered how to validate the escrow conditions on-chain, protecting the interests of both the depositor and recipient.

---

## 08/08/2024 - Class 6

In Class 6, we focused on the development of an Automated Market Maker (AMM) on Solana using the Anchor framework. An AMM is a decentralized exchange protocol that facilitates token swapping by utilizing liquidity pools and a mathematical formula to determine prices.

### Key Features

1. **Liquidity Pool Management**:
   - The AMM maintains liquidity pools where users can deposit pairs of tokens. These pools are essential for enabling decentralized trading. Liquidity providers earn fees from trades made within the pool, incentivizing them to contribute liquidity.

2. **Constant Product Formula (x * y = k)**:
   - The AMM uses the constant product formula, \(x \times y = k\), to maintain the balance between the two assets in the pool. This formula ensures that the product of the quantities of the two assets remains constant, automatically adjusting the price based on the available liquidity.

3. **Slippage Protection and Fee Mechanism**:
   - The AMM implements slippage protection to prevent significant price changes during large trades. A fee mechanism is also in place, which charges a small percentage on each trade, with the fees being distributed to liquidity providers. This helps stabilize the pool and protect traders from excessive slippage.

---

### 13/08/2024 - 15/08/2024 - Classes 7 - 9

During Classes 7 through 9, we concentrated on two primary areas: implementing NFT staking and crafting user stories for the capstone project.

### Key Concepts Covered

1. **NFT Staking**:
   - We implemented a program that allows users to stake their NFTs and earn rewards. The staking mechanism, built on Solana using the Anchor framework, includes key features such as NFT deposit and withdrawal, reward distribution based on staking duration, and support for multiple NFT collections. This program is designed to incentivize users to hold and engage with their NFTs by providing additional utility and rewards.

2. **Developing User Stories for the Capstone Project**:
   - Role played user stories to derive them in a better way finally.

---

### 20/08/2024 - Class 10
   - The Class 10 NFT Marketplace is a decentralized platform that allows users to list, delist, buy NFTS

### Key Concepts Covered
   1. Listing and Delisting NFTs: Users can list their NFTs for sale on the marketplace and delist them if they choose to remove them from sale.
   2. Buying NFTs: Buyers can purchase NFTs listed on the marketplace, with the platform handling the transfer of ownership and payment processing.

### 21/08/2024 - Class 11
   - In this class we covered a dice game that had randomness it will help in prediction markets 