import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { SportsHub } from "../target/types/sports_hub";
import { assert } from "chai";

describe("sports-hub", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SportsHub as Program<SportsHub>;

  // Generate random keypairs for players
  const player1 = web3.Keypair.generate();
  const player2 = web3.Keypair.generate();
  const house = provider.wallet;

  let vaultPda: web3.PublicKey;
  let vaultBump: number;

  // Define a test event and bets
  const eventId = new BN(1);  // Event ID is simply a number (1 in this case)
  const teamA = "Team A";
  const teamB = "Team B";
  let event: web3.Keypair;  // Store the event Keypair globally for reuse

  before(async () => {
    // Airdrop some SOL to player1 and player2 for testing
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(player1.publicKey, web3.LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(player2.publicKey, web3.LAMPORTS_PER_SOL)
    );

    // Derive the PDA for the vault
    [vaultPda, vaultBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), house.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the vault", async () => {
    await program.methods
      .initialize(new BN(web3.LAMPORTS_PER_SOL / 2)) // Deposit 0.5 SOL
      .accounts({
        house: house.publicKey,
        vault: vaultPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])  // No additional signers since the house is the wallet
      .rpc();

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.strictEqual(vaultBalance, web3.LAMPORTS_PER_SOL / 2, "Vault balance should be 0.5 SOL");
  });

  it("Creates a sports event", async () => {
    event = web3.Keypair.generate();  // Generate new keypair for the event

    const startTime = Math.floor(Date.now() / 1000) + 600; // Set the event start time to 10 minutes in the future

    await program.methods
      .createEvent(eventId, teamA, teamB, new BN(startTime))
      .accounts({
        event: event.publicKey,
        payer: house.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([event])
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.strictEqual(eventAccount.teamA, teamA, "Team A should be correct");
    assert.strictEqual(eventAccount.teamB, teamB, "Team B should be correct");
    assert.strictEqual(eventAccount.startTime.toNumber(), startTime, "Start time should be correct");
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

    await program.methods
      .placeBet(eventId, 0, new BN(web3.LAMPORTS_PER_SOL / 10)) // Bet 0.1 SOL on Team A
      .accounts({
        player: player1.publicKey,
        vault: vaultPda,
        event: event.publicKey,
        bet: betPda,                 // PDA for bet
        playerStats: playerStatsPda,  // PDA for playerStats
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([player1])  // Only player1 should sign
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

    await program.methods
      .placeBet(eventId, 1, new BN(web3.LAMPORTS_PER_SOL / 5)) // Bet 0.2 SOL on Team B
      .accounts({
        player: player2.publicKey,
        vault: vaultPda,
        event: event.publicKey,
        bet: betPda,                 // PDA for bet
        playerStats: playerStatsPda,  // PDA for playerStats
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([player2])  // Only player2 should sign
      .rpc();

    const eventAccount = await program.account.event.fetch(event.publicKey);
    assert.strictEqual(eventAccount.outcomeBBets.toString(), (web3.LAMPORTS_PER_SOL / 5).toString(), "Bet amount for Team B should be 0.2 SOL");
  });

  it("Resolves the event with Team A winning", async () => {
    await program.methods
      .resolveEvent(eventId, 0)  // Team A wins (outcome 0)
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
    const [betPda, betBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("bet"), event.publicKey.toBuffer(), player1.publicKey.toBuffer()],
      program.programId
    );
    const [playerStatsPda, playerStatsBump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("stats"), player1.publicKey.toBuffer()],
      program.programId
    );

    const balanceBefore = await provider.connection.getBalance(player1.publicKey);

    await program.methods
      .distributeRewards()
      .accounts({
        player: player1.publicKey,
        vault: vaultPda,
        event: event.publicKey,
        bet: betPda,                 // Use the same bet PDA from earlier
        playerStats: playerStatsPda,  // Player stats account (PDA)
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([player1])  // Only player1 should sign
      .rpc();

    const balanceAfter = await provider.connection.getBalance(player1.publicKey);
    assert(balanceAfter > balanceBefore, "Player 1 should have received a reward");
  });
});
