import cron from "node-cron";
import * as dotenv from "dotenv";
import { fetchCreatedEvents } from "./displayEvents";
import { createFplEventsOnChain } from "./createEvents";
import { fetchAndResolveEvents } from "./resolveEvents";
import { fetchAndDeleteEvents } from "./deleteEvents";

dotenv.config();

// Run event creation at startup
(async () => {
  console.log("Fetching and displaying created events...");
  await fetchCreatedEvents();
  console.log("Events fetched and displayed.");

})();

// Schedule task to create events every hour
cron.schedule("0 * * * *", async () => {
  console.log("Running scheduled task: Creating FPL events on-chain...");
  await createFplEventsOnChain();
  console.log("Scheduled task: Event creation completed.");
});

// Schedule task to display created events every 10 minutes
cron.schedule("10 * * * *", async () => {
  console.log("Running scheduled task: Fetching and displaying created events...");
  await fetchCreatedEvents();
  console.log("Scheduled task: Events fetched and displayed.");
});

// Schedule task to resolve events every 15 minutes
cron.schedule("15 * * * *", async () => {
  console.log("Running scheduled task: Resolving unresolved events...");
  await fetchAndResolveEvents();
  console.log("Scheduled task: Event resolution completed.");
});

// Schedule task to delete resolved events every 30 minutes
cron.schedule("30 * * * *", async () => {
  console.log("Running scheduled task: Deleting resolved events...");
  await fetchAndDeleteEvents();
  console.log("Scheduled task: Resolved events deleted.");
});
