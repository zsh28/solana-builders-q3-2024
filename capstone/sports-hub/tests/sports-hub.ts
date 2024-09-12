import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { SportsHub } from "../target/types/sports_hub";
import { assert } from "chai";

describe("sports-hub", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SportsHub as Program<SportsHub>;

  const player1 = web3.Keypair.generate();
  const player2 = web3.Keypair.generate();
  const house = provider.wallet;

  let vaultPda: web3.PublicKey;
  let vaultBump: number;
  const eventId = new BN(1);  // Event ID is simply a number (1 in this case)
  const teamA = "Team A";
  const teamB = "Team B";
  let event: web3.Keypair;

  before(async () => {
    // Airdrop SOL for player1 and player2
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(player1.publicKey, web3.LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(player2.publicKey, web3.LAMPORTS_PER_SOL)
    );
     // Derive the vault PDA
     [vaultPda, vaultBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), house.publicKey.toBuffer()],
      program.programId
    );

    console.log("Vault PDA:", vaultPda.toString());
  });

  it("Initializes the vault", async () => {
    //console log vault pda
    console.log("Vault PDA:", vaultPda.toString());
    console.log("House", house.publicKey.toString());
    await program.methods
      .initialize(new BN(web3.LAMPORTS_PER_SOL / 2)) // Deposit 0.5 SOL
      .accounts({
        house: house.publicKey,
      })
      .signers([])
      .rpc();

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.strictEqual(vaultBalance, web3.LAMPORTS_PER_SOL / 2, "Vault balance should be 0.5 SOL");
  });

  it("Creates a sports event", async () => {
    event = web3.Keypair.generate();

    const durationInSeconds = 60;  // 1 minute from now

    console.log("Setting event duration to 1 minute in the future.");

    // Create the sports event (only pass the duration now)
    await program.methods
      .createEvent(teamA, teamB, new BN(durationInSeconds))
      .accounts({
        event: event.publicKey,
        payer: house.publicKey,
      })
      .signers([event])
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    console.log("Event start time in contract:", eventAccount.startTime.toNumber());

    assert.strictEqual(eventAccount.teamA, teamA, "Team A should be correct");
    assert.strictEqual(eventAccount.teamB, teamB, "Team B should be correct");
  });

  it("Player 1 places a bet on Team A", async () => {
    const [betPda, betBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("bet"), event.publicKey.toBuffer(), player1.publicKey.toBuffer()],
      program.programId
    );
    const [playerStatsPda, playerStatsBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("stats"), player1.publicKey.toBuffer()],
      program.programId
    );

    console.log("Bet PDA:", betPda.toString());
    console.log("Player Stats PDA:", playerStatsPda.toString());

    await program.methods
      .placeBet(eventId, 0, new BN(web3.LAMPORTS_PER_SOL / 10)) // Bet 0.1 SOL on Team A
      .accounts({
        player: player1.publicKey,
        vault: vaultPda,
        event: event.publicKey,
      })
      .signers([player1])
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.strictEqual(eventAccount.outcomeABets.toString(), (web3.LAMPORTS_PER_SOL / 10).toString(), "Bet amount for Team A should be 0.1 SOL");
  });

  it("Player 2 places a bet on Team B", async () => {
    const [betPda, betBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("bet"), event.publicKey.toBuffer(), player2.publicKey.toBuffer()],
      program.programId
    );
    const [playerStatsPda, playerStatsBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("stats"), player2.publicKey.toBuffer()],
      program.programId
    );

    console.log("Bet PDA:", betPda.toString());
    console.log("Player Stats PDA:", playerStatsPda.toString());

    await program.methods
      .placeBet(eventId, 1, new BN(web3.LAMPORTS_PER_SOL / 5)) // Bet 0.2 SOL on Team B
      .accounts({
        player: player2.publicKey,
        vault: vaultPda,
        event: event.publicKey,
      })
      .signers([player2])
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.strictEqual(eventAccount.outcomeBBets.toString(), (web3.LAMPORTS_PER_SOL / 5).toString(), "Bet amount for Team B should be 0.2 SOL");
  });

  it("Wait for event to start", async () => {
    console.log("Waiting for the event to start...");

    const waitTimeInMilliseconds = 2 * 1000; // 1 minute in milliseconds
    const intervalInMilliseconds = 1000; // Countdown interval (1 second)

    let remainingTime = waitTimeInMilliseconds / 1000; // Remaining time in seconds

    const interval = setInterval(() => {
        remainingTime -= 1;
        console.log(`Time left: ${Math.floor(remainingTime / 60)} minutes ${remainingTime % 60} seconds`);
    }, intervalInMilliseconds);

    // Wait for the set time, and then clear the interval
    await new Promise(resolve => setTimeout(() => {
        clearInterval(interval);
        resolve(true);
    }, waitTimeInMilliseconds));

    console.log("Event should now start.");
  });

  it("Resolves the event with Team A winning", async () => {
    await program.methods
      .resolveEvent(eventId, 0)  // Team A wins
      .accounts({
        admin: house.publicKey,
        event: event.publicKey,
      })
      .signers([])
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.strictEqual(eventAccount.winningOutcome, 0, "Winning outcome should be Team A (0)");
  });

  it("Player 1 claims reward for betting on Team A", async () => {
    // Derive the bet PDA for player1
    const [betPda, betBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("bet"), event.publicKey.toBuffer(), player1.publicKey.toBuffer()],
      program.programId
    );
  
    // Derive the player stats PDA for player1
    const [playerStatsPda, playerStatsBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("stats"), player1.publicKey.toBuffer()],
      program.programId
    );
  

    console.log("Bet PDA:", betPda.toString());
    console.log("Player Stats PDA:", playerStatsPda.toString());
    console.log("Vault PDA:", vaultPda.toString());
    //console log house 
    console.log("House PDA:", house.publicKey.toString());
  
    await program.methods
      .distributeRewards(eventId)
      .accounts({
        player: player1.publicKey,
        event: event.publicKey,
        bet: betPda,
        playerStats: playerStatsPda,
        house: house.publicKey,  
      })
      .signers([player1])
      .rpc();
  });
  
  
});
