# Sports-Hub

**Sports-Hub** is a decentralized sports betting application built on the Solana blockchain using the Anchor framework. The platform allows users to create and participate in bets on sports events, with functionality for placing bets, resolving events, and distributing rewards.

## Features

- **Event Creation**: Admins can create events by providing details such as the teams and kickoff time.
- **Betting**: Users can place bets on sports events, choosing outcomes such as Team A, Team B, or a draw.
- **Event Resolution**: Once a sports event concludes, the admin can resolve the event by determining the winning outcome.
- **Rewards Distribution**: After event resolution, the platform distributes rewards to users who placed winning bets, with a small platform fee applied.
- **Admin Controls**: Admins have the ability to delete events after ensuring all rewards have been claimed.
  
## Smart Contract Overview

### `lib.rs`

This is the entry point to the program, where modules and external dependencies are declared. The `sports_hub` program includes multiple instructions to manage sports events and bets.

### Key Instructions

1. **Initialize**: Initializes the platform with a vault to store bet amounts in SOL.
2. **Create Event**: Creates a new sports event with details fetched from the FPL (Fantasy Premier League) API.
3. **Place Bet**: Allows users to place bets on the outcome of a sports event. Bets can be placed on Team A, Team B, or a draw.
4. **Resolve Event**: Resolves an event by setting the winning outcome, which determines the users who won the bet.
5. **Distribute Rewards**: Handles the reward distribution to winning participants, deducting a platform fee before distributing winnings.
6. **Delete Event**: Admins can delete events, but only if the event has been resolved and all rewards have been claimed.

### State Structures

- **Event**: Represents a sports event, including teams, start time, total bets, outcome details, and resolution status.
- **Bet**: Represents a bet placed by a user on an event, storing the user’s details, bet amount, and outcome choice.
- **PlayerStats**: Tracks the total bets placed and winnings for each player on the platform.

### Program Flow

1. **Initialize**: Set up the vault to manage SOL funds for bets.
2. **Create Event**: Admins initialize a sports event, passing in relevant information.
3. **Place Bet**: Users place their bets by choosing an outcome and transferring SOL to the vault.
4. **Resolve Event**: The event is resolved by the admin, and the winning outcome is recorded.
5. **Distribute Rewards**: After the event is resolved, winnings are distributed to users who bet on the correct outcome.
6. **Delete Event**: Once all rewards have been claimed, the event and associated bet accounts are deleted.

### Error Handling

The program uses custom errors to handle conditions like invalid event IDs, insufficient funds, invalid outcomes, and unresolved events. Key error scenarios include:

- **InvalidEvent**: Thrown when an event ID does not match.
- **EventAlreadyResolved**: Thrown when trying to resolve an already-resolved event.
- **BettingClosed**: Thrown when users attempt to bet after the event's start time.
- **InvalidBetAmount**: Thrown when a bet amount is zero or less.
- **BetLost**: Thrown when a player loses a bet and cannot claim rewards.

## Program ID
- This program is deployed using the [Anchor CLI](https://github.com/coral-xyz/anchor) tool.
- The program ID is: 58BwTBP6UGRZVYgt1tD6ijoQdrFLAybuAB1kHnT9m29N
- It is deployed on the Solana Devnet.