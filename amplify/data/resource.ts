import { type ClientSchema, a, defineData } from "@aws-amplify/backend";
import { postUpdateStature } from "../functions/post-update-stature/recource";

const schema = a
  .schema({
    User: a
      .model({
        wallet: a.string().required(), // The raw Public Key
        name: a.string(),
        bio: a.string(),
        avatarUrl: a.string(),
        bannerUrl: a.string(),
        website: a.string(),
        twitter: a.string(),
        // We store a cache of the stature for fast sorting/searching in the UI
        stature: a.string(),
        isVerified: a.boolean().default(false),
        programs: a.hasMany("ProgramUser", "userWallet"),
      })
      .identifier(["wallet"])
      .authorization((allow) => [
        allow.guest().to(["read"]),
        allow.owner().to(["create", "update", "delete"]),
      ]),

    Program: a
      .model({
        id: a.string().required(), // The Program PDA
        name: a.string().required(),

        company: a.string(), // The admin Pubkey from Anchor
        website: a.string(),

        description: a.string(),
        logoUrl: a.string(),
        industry: a.string(),
        location: a.string(),
        isSuspended: a.boolean().default(false),
        isVerified: a.boolean().default(false),
        // Weight and record cap cache for the Find page
        weight: a.string(),

        recordCount: a.string(),
        stature: a.string(),
        totalPositive: a.string(),
        totalNegative: a.string(),

        users: a.hasMany("ProgramUser", "programPda"),
      })
      .authorization((allow) => [
        allow.guest().to(["read"]),
        allow.owner().to(["create", "update", "delete"]),
      ]),

    // 3. The Join Table
    ProgramUser: a
      .model({
        role: a.string(), // Bonus: Store metadata like 'admin', 'employee', 'founder'

        userWallet: a.string().required(),
        programPda: a.string().required(),
        user: a.belongsTo("User", "userWallet"),
        program: a.belongsTo("Program", "programPda"),
      })
      .secondaryIndexes((index) => [
        index("userWallet").queryField("programsByUser"),
        index("programPda").queryField("usersByProgram"),
      ])

      .authorization((allow) => [
        allow.guest().to(["read"]),
        // Usually, you'd want logic where the User or Program Admin can manage this
        // allow.authenticated().to(["read"]),
        allow.owner().to(["create", "update", "delete"]),
      ]),

    // This bridges the on-chain StatureRecord with off-chain context
    StatureRecord: a
      .model({
        signature: a.string().required(),
        userWallet: a.string().required(),
        programPda: a.string().required(),
        programSourceAccountPda: a.string().required(),

        statureChange: a.string().required(), // big int

        //   txAmount: a.string(), // big int, maybe have booleans

        // title: a.string(), // "Senior Smart Contract Audit"
        memo: a.string(), // Long form details not fit for Solana

        onChainTimestamp: a.datetime().required(),
      })
      .identifier(["signature"])
      // Secondary indexes to quickly find all records for a user or company
      .secondaryIndexes((index) => [
        index("userWallet").queryField("listRecordsByUser"),
        index("programPda").queryField("listRecordsByProgram"),
      ])
      .authorization((allow) => [
        allow.guest().to(["read"]),
        // Only the company that issued the record can edit the metadata
        allow.owner().to(["create", "update"]),
      ]),
  })
  .authorization((allow) => [allow.resource(postUpdateStature).to(["mutate"])]);
export type Schema = ClientSchema<typeof schema>;

export const data = defineData({
  schema,
  authorizationModes: {
    // We use identityPool for guest access (Search)
    // and userPool for the 'owner' logic (Claiming Identity)
    defaultAuthorizationMode: "identityPool",
    apiKeyAuthorizationMode: {
      expiresInDays: 30,
    },
  },
});
