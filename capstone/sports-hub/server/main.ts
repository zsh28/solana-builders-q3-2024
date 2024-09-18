import axios from "axios";
import cron from "node-cron";
import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import dotenv from "dotenv";

dotenv.config(); // Load environment variables

// Ensure environment variables exist
if (!process.env.SOLANA_RPC_URL || !process.env.RESOLVER_SECRET_KEY) {
  throw new Error("Missing required environment variables");
}

// Load the Solana keypair (resolver) from environment variables
const resolverKeypair = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(process.env.RESOLVER_SECRET_KEY))
);

// Set up Solana connection
const connection = new Connection(process.env.SOLANA_RPC_URL, "confirmed");

// Solana program ID (replace with your program ID)
const programId = new PublicKey("7zqecNY5KfdS8KCyNYBrtHya8SmxXyUta3hnLEvBCZxz");

// FPL API endpoints
const TEAM_URL = "https://fantasy.premierleague.com/api/bootstrap-static/";
const FIXTURES_URL = "https://fantasy.premierleague.com/api/fixtures/";

// Store processed event IDs to avoid recreating or reresolving
const createdEvents: Set<number> = new Set();
const resolvedEvents: Set<number> = new Set();

// Type for FPL Team
interface FplTeam {
  id: number;
  name: string;
}

// Type for FPL Fixture
interface FplFixture {
  id: number;
  team_a: number;
  team_h: number;
  team_a_score: number | null;
  team_h_score: number | null;
  finished: boolean;
  started: boolean;
  kickoff_time: string;
}

// Map team IDs to names
async function fetchTeamNameMapping(): Promise<Record<number, string>> {
  try {
    const response = await axios.get(TEAM_URL);
    const teams: FplTeam[] = response.data.teams;
    const teamMap: Record<number, string> = {};
    teams.forEach((team) => {
      teamMap[team.id] = team.name;
    });
    return teamMap;
  } catch (error) {
    console.error("Error fetching team data from FPL API:", error);
    return {};
  }
}

// Fetch upcoming matches for today and next day (limit to 10)
async function fetchUpcomingMatches(): Promise<FplFixture[]> {
  const now = new Date();
  const upcomingWindow = new Date(now.getTime() + 24 * 60 * 60 * 1000); // Up to 24 hours in the future

  try {
    const response = await axios.get(FIXTURES_URL);
    const fixtures: FplFixture[] = response.data;

    // Filter for matches that start today or tomorrow and have not started yet
    const upcomingMatches = fixtures.filter((fixture) => {
      const kickoffTime = new Date(fixture.kickoff_time);
      return (
        kickoffTime >= now &&
        kickoffTime <= upcomingWindow &&
        !fixture.started &&
        !createdEvents.has(fixture.id)
      );
    });

    // Limit to the next 10 matches
    return upcomingMatches.slice(0, 10);
  } catch (error) {
    console.error("Error fetching fixtures from FPL API:", error);
    return [];
  }
}

// Fetch finished matches for today and the previous day (limit to 10)
async function fetchFinishedMatches(): Promise<FplFixture[]> {
  const now = new Date();
  const previousWindow = new Date(now.getTime() - 24 * 60 * 60 * 1000); // Past 24 hours

  try {
    const response = await axios.get(FIXTURES_URL);
    const fixtures: FplFixture[] = response.data;

    // Filter for matches that finished in the last 24 hours and haven't been resolved yet
    const finishedMatches = fixtures.filter((fixture) => {
      const kickoffTime = new Date(fixture.kickoff_time);
      return (
        fixture.finished &&
        kickoffTime >= previousWindow &&
        kickoffTime <= now &&
        !resolvedEvents.has(fixture.id)
      );
    });

    // Limit to the last 10 matches
    return finishedMatches.slice(0, 10);
  } catch (error) {
    console.error("Error fetching finished matches from FPL API:", error);
    return [];
  }
}

// Create an event on-chain when a match is about to start
async function createMatchEvent(
  fplEventId: number,
  teamAName: string,
  teamHName: string,
  durationInSeconds: number
): Promise<void> {
  const transaction = new Transaction().add(
    new TransactionInstruction({
      keys: [
        {
          pubkey: resolverKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
      ],
      programId: programId,
      data: Buffer.from(
        JSON.stringify({
          fplEventId: fplEventId,
          teamA: teamAName,
          teamB: teamHName,
          durationInSeconds: durationInSeconds,
        })
      ),
    })
  );

  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      resolverKeypair,
    ]);
    console.log(
      `Event created on-chain for FPL match ID ${fplEventId} (${teamHName} vs ${teamAName}) with signature: ${signature}`
    );
    createdEvents.add(fplEventId); // Mark the event as created
  } catch (error) {
    console.error("Error creating event on-chain:", error);
  }
}

// Resolve event on-chain using FPL match ID
async function resolveEventOnChain(
  fplEventId: number,
  winningOutcome: number
): Promise<void> {
  const transaction = new Transaction().add(
    new TransactionInstruction({
      keys: [
        {
          pubkey: resolverKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
      ],
      programId: programId,
      data: Buffer.from(
        JSON.stringify({
          eventId: fplEventId,
          winningOutcome: winningOutcome,
        })
      ),
    })
  );

  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      resolverKeypair,
    ]);
    console.log(
      `Event with FPL match ID ${fplEventId} resolved on-chain with signature: ${signature}`
    );
    resolvedEvents.add(fplEventId); // Mark the event as resolved
  } catch (error) {
    console.error("Error resolving event on-chain:", error);
  }
}

// Determine the winning outcome based on match result
function determineWinningOutcome(match: FplFixture): number {
  if (match.team_a_score! > match.team_h_score!) {
    return 0; // Team A wins
  } else if (match.team_h_score! > match.team_a_score!) {
    return 1; // Team H wins
  } else {
    return 2; // Draw
  }
}

// Monitor for matches that are starting soon (today or next day)
async function monitorUpcomingMatches(): Promise<void> {
  const teamMap = await fetchTeamNameMapping();
  const upcomingMatches = await fetchUpcomingMatches();

  if (upcomingMatches.length > 0) {
    console.log("Creating events for upcoming matches...");

    for (const match of upcomingMatches) {
      const teamAName = teamMap[match.team_a] || "Unknown";
      const teamHName = teamMap[match.team_h] || "Unknown";
      console.log(`Creating event for match: ${teamHName} vs ${teamAName}`);

      // Create event on-chain
      await createMatchEvent(match.id, teamAName, teamHName, 3600); // 1 hour duration for this example
    }
  } else {
    console.log("No upcoming matches found.");
  }
}

// Monitor for finished matches and resolve them
async function monitorFinishedMatches(): Promise<void> {
  const teamMap = await fetchTeamNameMapping();
  const finishedMatches = await fetchFinishedMatches();

  if (finishedMatches.length > 0) {
    console.log("Resolving finished matches...");

    for (const match of finishedMatches) {
      const teamAName = teamMap[match.team_a] || "Unknown";
      const teamHName = teamMap[match.team_h] || "Unknown";

      // Determine the winning outcome
      const winningOutcome = determineWinningOutcome(match);

      console.log(
        `Resolving match: ${teamHName} vs ${teamAName} | Winning outcome: ${winningOutcome}`
      );

      // Resolve the event on-chain
      await resolveEventOnChain(match.id, winningOutcome);
    }
  } else {
    console.log("No finished matches to resolve.");
  }
}

// Schedule the task to create events before matches start (runs every 10 minutes)
cron.schedule(
  "*/10 * * * *",
  async () => {
    console.log("Checking for upcoming matches...");
    await monitorUpcomingMatches();
  },
  {
    scheduled: true,
    timezone: "UTC",
  }
);

// Schedule the task to resolve finished matches (runs every minute)
cron.schedule(
  "* * * * *",
  async () => {
    console.log("Checking for finished matches...");
    await monitorFinishedMatches();
  },
  {
    scheduled: true,
    timezone: "UTC",
  }
);

// Run immediately on start for both upcoming matches and finished matches
(async () => {
  await monitorUpcomingMatches();
  await monitorFinishedMatches();
})();
