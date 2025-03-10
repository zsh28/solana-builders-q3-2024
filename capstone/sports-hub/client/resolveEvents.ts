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

// Function to resolve an event
export async function resolveEvent(eventPublicKey: web3.PublicKey, eventId: BN, winningOutcome: number) {
  try {
    const txSignature = await program.methods
      .resolveEvent(eventId, winningOutcome) // Pass the event ID and winning outcome
      .accounts({
        event: eventPublicKey,   // The event account being resolved
      })
      .signers([]) // No additional signers needed as the house wallet is the signer
      .rpc();

    console.log(`Event resolved on chain. Transaction Signature: ${txSignature}`);
  } catch (error) {
    console.error("Error resolving event:", error);
  }
}

// Function to fetch match results from the FPL API and determine which team won
export async function fetchMatchResult(fplEventId: number): Promise<number | null> {
  try {
    console.log(`Fetching result for FPL event ID: ${fplEventId}`); // Log event ID
    const response = await axios.get(`https://fantasy.premierleague.com/api/fixtures/`);
    const fixtures = response.data;

    // Find the event by its FPL event ID
    const match = fixtures.find((fixture: any) => fixture.id === fplEventId);

    if (!match) {
      console.log(`No match found for FPL event ID: ${fplEventId}`);
      return null;
    }

    if (!match.finished) {
        //log var match
      console.log(`Match not finished: ${match.team_h_score} - ${match.team_a_score}`);
      console.log(`Match with ID ${fplEventId} is not finished.`);
      return null;
    }

    console.log(`Match finished: ${match.team_h_score} - ${match.team_a_score}`); // Log scores

    const teamAScore = match.team_h_score;
    const teamBScore = match.team_a_score;

    if (teamAScore > teamBScore) {
      return 0; // Team A wins
    } else if (teamBScore > teamAScore) {
      return 1; // Team B wins
    } else {
      console.log(`Match with ID ${fplEventId} ended in a draw.`);
      return 2; // Draw
    }
  } catch (error) {
    console.error("Error fetching match result from FPL API:", error);
    return null;
  }
}

// Function to fetch unresolved events and resolve them based on FPL API results
export async function fetchAndResolveEvents() {
  try {
    console.log("Fetching unresolved events from the blockchain...");

    const events = await program.account.event.all();
    console.log(`Found ${events.length} events.`); // Log the number of events found

    for (const event of events) {
      console.log(`Checking event: ${event.publicKey.toString()} - ${event.account.teamA} vs ${event.account.teamB}`);
      
      // Assuming that events where winningOutcome is null or undefined are unresolved
      if (event.account.winningOutcome === null || event.account.winningOutcome === undefined) {
        console.log(`Event unresolved: ${event.account.teamA} vs ${event.account.teamB}`);
        
        const fplEventId = event.account.eventId.toNumber(); // Fetch the FPL event ID from the event account
        console.log(`FPL Event ID from blockchain: ${fplEventId}`); // Log FPL event ID
        
        const winningOutcome = await fetchMatchResult(fplEventId);

        // If we got a valid result from the API, resolve the event
        if (winningOutcome !== null) {
          console.log(`Resolving event: ${event.account.teamA} vs ${event.account.teamB} with outcome: ${winningOutcome}`);
          await resolveEvent(event.publicKey, event.account.eventId, winningOutcome);
        } else {
          console.log(`Could not resolve event: ${event.account.teamA} vs ${event.account.teamB}`);
        }
      } else {
        console.log(`Event already resolved: ${event.account.teamA} vs ${event.account.teamB}`);
      }
    }

    console.log("All unresolved events have been processed.");
  } catch (error) {
    console.error("Error fetching or resolving events:", error);
  }
}
