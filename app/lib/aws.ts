import { createServerRunner } from "@aws-amplify/adapter-nextjs";
import outputs from "@/amplify_outputs.json";
import { generateClient } from "aws-amplify/data";
import type { Schema } from "@/amplify/data/resource";
import { UpdateStatureEvent } from "../generated/stature";

export const AWS_STATURE_API_URL = outputs.custom.API.StatureApi.endpoint;
export interface PostUpdateStatureEventRequest extends UpdateStatureEvent {
  signature: string;
}

export const { runWithAmplifyServerContext, createAuthRouteHandlers } =
  createServerRunner({
    config: outputs,

    runtimeOptions: {
      cookies: {
        domain: ".quran600.com", // making cookies available to all subdomains
        sameSite: "strict",
        maxAge: 60 * 60 * 24 * 7, // 7 days
      },
    },
  });

export const client = generateClient<Schema>();

// import { generateServerClientUsingReqRes } from "@aws-amplify/adapter-nextjs/data";

// export const client = generateServerClientUsingReqRes({
//   authMode: "apiKey",
//   config: outputs,
// });
