import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stature } from "@/anchor/target/types/stature";
import axios from "axios";
import { POST_UPATE_STATURE_API_URL, PostUpdateStatureEventRequest } from "./app/lib/aws";
import { address } from "@solana/kit";

// 1. Setup Provider & Program
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.StatureNetwork as Program<Stature>;

console.log("Listening for Stature events...");

// 2. Add the WebSocket Listener
program.addEventListener(
  "updateStatureEvent",
  async (event, slot, signature) => {
    console.log(
      `Event caught! User: ${event.userWallet.toString()} | Amount: ${event.stature}`
    );

    try {
      const req: PostUpdateStatureEventRequest = {
        signature: signature,

        userWallet: address(event.userWallet.toString()),
        program: address(event.program.toString()),
        programSourceAccount: address(event.programSourceAccount.toString()),
        stature: event.stature,
        memo: event.memo,
        timestamp: event.timestamp,
      };

      // 3. POST to your Amplify Lambda URL
      await axios.post(
        POST_UPATE_STATURE_API_URL,
       req,
        {
          headers: { "x-api-key": "YOUR_SECRET_SECRET" }, // Simple security
        }
      );
      console.log("Successfully sent to Lambda");
    } catch (err) {
      console.error("Failed to ping Lambda:", err);
    }
  }
);
