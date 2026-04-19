import { defineFunction } from "@aws-amplify/backend";

export const postUpdateStature = defineFunction({
  name: "post-update-stature",
  entry: "./handler.ts",
});
