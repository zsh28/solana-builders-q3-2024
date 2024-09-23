import { Connection } from "@solana/web3.js";
import dotenv from "dotenv";

dotenv.config();

(async () => {
  const connection = new Connection(process.env.SOLANA_RPC_URL, "confirmed");
  const version = await connection.getVersion();
  console.log("Solana RPC Version:", version);
})();
