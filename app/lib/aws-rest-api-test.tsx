"use client";
import { address } from "@solana/kit";
import { POST_UPATE_STATURE_API_URL, PostUpdateStatureEventRequest } from "./aws";

export default function PostUpdateStature_Btn() {
  const onPress = async () => {
    try {
      const event: PostUpdateStatureEventRequest = {
        userWallet: address("user ID"),
        program: address("program ID"),
        programSourceAccount: address("source ID"),
        stature: BigInt(300),
        memo: 'this is uuid or reason for this transaction: Contract between user 1 and user 2 complete ', 
        timestamp: BigInt(new Date().getTime()),
        signature: "signature id",
      };

      const res = await fetch(POST_UPATE_STATURE_API_URL,  {
        method: "POST",
        body: JSON.stringify(event),
      });

      console.log("success", res);
    } catch (e) {
      console.log("err", e);
    }
  };

  return (
    <button
      onClick={onPress}
      className="mt-34 rounded-full bg-black px-5 py-2 text-xs font-bold text-white transition-all hover:scale-105 dark:bg-white dark:text-black"
    >
      Test This thing
    </button>
  );
}
