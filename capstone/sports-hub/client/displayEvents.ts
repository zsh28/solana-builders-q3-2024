import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SportsHub } from "../target/types/sports_hub";
import * as dotenv from "dotenv";
dotenv.config();

// Set up the provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.SportsHub as Program<SportsHub>;

// Fetch and display created events from the blockchain
export async function fetchCreatedEvents() {
    try {
      console.log("Fetching created events from the blockchain..."); // Add this log
      const events = await program.account.event.all();
  
      if (events.length === 0) {
        console.log("No events found.");
        return;
      }
  
      console.log("Displaying created events:");
      events.forEach((event) => {
        console.log(`
          Event ID: ${event.publicKey.toString()},
          Team A: ${event.account.teamA},
          Team B: ${event.account.teamB},
          Start Time: ${new Date(event.account.startTime.toNumber() * 1000)},
          Bets on Team A: ${event.account.outcomeABets.toString()},
          Bets on Team B: ${event.account.outcomeBBets.toString()},
          Winning Outcome: ${event.account.winningOutcome ?? "Not resolved yet"}
        `);
      });
    } catch (error) {
      console.error("Error fetching events:", error);
    }
  }
  