"use client";
import { address } from "@solana/kit";
import { AWS_STATURE_API_URL, PostUpdateStatureEventRequest } from "./aws";

export default function PostUpdateStature_Btn() {
  const onPress = async () => {
    try {
      console.log("going for it");

      const event: PostUpdateStatureEventRequest = {
        user: address("user ID"),
        program: address("program ID"),
        source: address("source ID"),
        amount: BigInt(300),
        timestamp: BigInt(new Date().getTime()),
        signature: "signature id",
      };

      const res = await fetch(AWS_STATURE_API_URL + "update-stature", {
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
