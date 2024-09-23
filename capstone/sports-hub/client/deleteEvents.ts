import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { SportsHub } from "../target/types/sports_hub";
import * as dotenv from "dotenv";
dotenv.config();

// Set up the provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.SportsHub as Program<SportsHub>;
const house = provider.wallet;

// Function to check if all rewards have been claimed
function allRewardsClaimed(event: any): boolean {
  const totalBets = event.totalBets.toNumber();
  const claimedBets = event.outcomeABets.toNumber() + event.outcomeBBets.toNumber() + event.drawBets.toNumber();
  return claimedBets === totalBets;
}

// Function to fetch all Bet accounts associated with an event
async function fetchBetsForEvent(eventPublicKey: web3.PublicKey): Promise<{ publicKey: web3.PublicKey, account: any }[]> {
  try {
    const bets = await program.account.bet.all([
      {
        memcmp: {
          offset: 8, // Offset to the event's public key in the Bet account
          bytes: eventPublicKey.toBase58(),
        },
      },
    ]);

    return bets;
  } catch (error) {
    console.error("Error fetching bets for event:", error);
    return [];
  }
}

// Function to delete a bet account
export async function deleteBet(betPublicKey: web3.PublicKey, playerPublicKey: web3.PublicKey) {
  try {
    const txSignature = await program.methods
      .deleteEvent()  // Assuming deleteEvent will also clean up associated bet accounts
      .accounts({
        admin: house.publicKey,      // Admin or house wallet
        player: playerPublicKey,     // The player associated with the bet
      })
      .signers([])                   // The house wallet signs the transaction
      .rpc();

    console.log(`Bet deleted on-chain. Transaction Signature: ${txSignature}`);
  } catch (error) {
    console.error("Error deleting bet:", error);
  }
}

// Function to delete an event after deleting all associated bets
export async function deleteEvent(eventPublicKey: web3.PublicKey) {
  try {
    const bets = await fetchBetsForEvent(eventPublicKey);
    
    // Delete all associated bet accounts
    for (const bet of bets) {
      console.log(`Deleting bet: ${bet.publicKey.toString()} for event: ${eventPublicKey.toString()}`);
      await deleteBet(bet.publicKey, bet.account.user);
    }

    // Now delete the event
    const txSignature = await program.methods
      .deleteEvent()
      .accounts({
        admin: house.publicKey,       // Admin or house wallet
        event: eventPublicKey,        // The event to be deleted
      })
      .signers([])                    // The house wallet signs the transaction
      .rpc();

    console.log(`Event deleted on-chain. Transaction Signature: ${txSignature}`);
  } catch (error) {
    console.error("Error deleting event:", error);
  }
}

// Function to fetch resolved events and delete them if all rewards have been claimed
export async function fetchAndDeleteEvents() {
  try {
    console.log("Fetching events from the blockchain...");

    const events = await program.account.event.all();
    console.log(`Found ${events.length} events.`); // Log the number of events found

    for (const event of events) {
      console.log(`Checking event: ${event.publicKey.toString()} - ${event.account.teamA} vs ${event.account.teamB}`);
      
      // Ensure the event is resolved
      if (event.account.resolved) {
        console.log(`Event resolved: ${event.account.teamA} vs ${event.account.teamB}`);
        
        // Check if all rewards have been claimed
        if (allRewardsClaimed(event.account)) {
          console.log(`All rewards claimed for event: ${event.account.teamA} vs ${event.account.teamB}`);
          
          // Delete the event and its associated bets
          await deleteEvent(event.publicKey);
        } else {
          console.log(`Rewards not fully claimed for event: ${event.account.teamA} vs ${event.account.teamB}`);
        }
      } else {
        console.log(`Event not resolved yet: ${event.account.teamA} vs ${event.account.teamB}`);
      }
    }

    console.log("All deletable events have been processed.");
  } catch (error) {
    console.error("Error fetching or deleting events:", error);
  }
}
