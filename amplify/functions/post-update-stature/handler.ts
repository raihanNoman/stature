import type { Handler } from "aws-lambda";

import { type Schema } from "../../data/resource";
import { Amplify } from "aws-amplify";
import { generateClient } from "aws-amplify/data";
import { getAmplifyDataClientConfig } from "@aws-amplify/backend/function/runtime";
import { env } from "$amplify/env/post-update-stature";

const { resourceConfig, libraryOptions } =
  await getAmplifyDataClientConfig(env);
Amplify.configure(resourceConfig, libraryOptions);
const client = generateClient<Schema>();

// Define the shape of your Anchor event
interface PostUpdateStatureEventRequest {
  signature: string;

  userWallet: string;
  program: string;
  programSourceAccount: string;
  stature: bigint;
  memo: string;
  timestamp: bigint;
}

async function createEntry(event: PostUpdateStatureEventRequest) {
  const timestamp = Number(event.timestamp) * 1000;

  const { data, errors } = await client.models.StatureRecord.update({
    signature: event.signature,
    userWallet: event.userWallet,
    programPda: event.program,
    programSourceAccountPda: event.programSourceAccount,

    statureChange: event.stature.toString(),

    onChainTimestamp: new Date(timestamp).toISOString(),
  });

  if (!data || errors) {
    console.log("❌ StatureRecord.create", errors);
    throw "graph ql error";
  }

  console.log("✅ created new StatureRecord", data);
  return data;
}

export const handler: Handler = async (event) => {
  try {
    const body: PostUpdateStatureEventRequest = await JSON.parse(event.body);

    console.log("Processing Stature for:", body);

    await createEntry(body);
  } catch (e) {
    console.log("err creating stature record", e);
  }

  return {
    headers: {
      "Access-Control-Allow-Origin": "*", // Or "http://localhost:3000"
      "Access-Control-Allow-Headers": "Content-Type",
      "Access-Control-Allow-Methods": "OPTIONS,POST",
    },

    statusCode: 200,
    body: JSON.stringify({ message: "Stature recorded!" }),
  };
};
