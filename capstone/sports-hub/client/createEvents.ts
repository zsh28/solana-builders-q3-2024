import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { SportsHub } from "../target/types/sports_hub";
import axios from "axios";
import * as dotenv from "dotenv";
dotenv.config();

// Set up the provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.SportsHub as Program<SportsHub>;
const house = provider.wallet;

// Fetch FPL events (fixtures) from the FPL API
export async function fetchFplFixtures() {
  try {
    const response = await axios.get("https://fantasy.premierleague.com/api/fixtures/");
    return response.data;
  } catch (error) {
    console.error("Error fetching FPL fixtures:", error);
    return [];
  }
}

// Fetch team data from the FPL bootstrap API
export async function fetchTeamData() {
  try {
    const response = await axios.get("https://fantasy.premierleague.com/api/bootstrap-static/");
    const teams = response.data.teams;

    // Create a map to quickly find team names by team ID
    const teamMap = new Map();
    teams.forEach((team: any) => {
      teamMap.set(team.id, team.name);
    });

    return teamMap;
  } catch (error) {
    console.error("Error fetching team data:", error);
    return new Map();
  }
}

// Check if an event already exists on-chain by fpl_event_id
export async function eventExists(fplEventId: BN): Promise<boolean> {
  try {
    const events = await program.account.event.all();
    return events.some(event => event.account.eventId.eq(fplEventId));
  } catch (error) {
    console.error("Error checking if event exists:", error);
    return false;
  }
}

// Create an event on the blockchain using the FPL event data
export async function createFplEvent(eventData: any, teamMap: Map<number, string>) {
  const fplEventId = new BN(eventData.id);

  // Check if the event already exists on-chain
  if (await eventExists(fplEventId)) {
    console.log(`Event with FPL ID ${fplEventId.toString()} already exists. Skipping...`);
    return;
  }

  const event = web3.Keypair.generate();
  const teamA = teamMap.get(eventData.team_h); // Get home team name
  const teamB = teamMap.get(eventData.team_a); // Get away team name

  if (!teamA || !teamB) {
    console.error(`Could not find team names for event: ${eventData.id}`);
    return;
  }

  // Convert the kickoff time to a UNIX timestamp
  const kickoffTime = Math.floor(new Date(eventData.kickoff_time).getTime() / 1000);

  console.log(`Creating event: ${teamA} vs ${teamB} with kickoff time: ${eventData.kickoff_time}`);

  try {
    const txSignature = await program.methods
      .createEvent(fplEventId, teamA, teamB, new BN(kickoffTime)) // Pass kickoff time here as a BN
      .accounts({
        event: event.publicKey,
        payer: house.publicKey,
      })
      .signers([event])
      .rpc();

    console.log(`Event created on chain: ${teamA} vs ${teamB}`);
    console.log(`Transaction Signature: ${txSignature}`);
  } catch (error) {
    console.error("Error creating event:", error);
  }
}

// Main function to fetch FPL events and create them on-chain
export async function createFplEventsOnChain() {
  const fixtures = await fetchFplFixtures();
  const teamMap = await fetchTeamData(); // Fetch team data once
  let eventsCreated = 0;

  for (const fixture of fixtures) {
    // Limit event creation to the first 10 events
    if (eventsCreated >= 10) {
      console.log("Limit of 10 events reached. Stopping creation.");
      break;
    }

    // Filter out events that have already started or are invalid
    if (fixture.started === false && fixture.kickoff_time) {
      await createFplEvent(fixture, teamMap);
      eventsCreated++;
    }
  }

  console.log(`${eventsCreated} events created.`);
}
