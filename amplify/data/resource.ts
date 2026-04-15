import { type ClientSchema, a, defineData } from '@aws-amplify/backend';

const schema = a.schema({
  // The 'id' here will be the Solana User PDA string
  UserProfile: a
    .model({
      id: a.string().required(), // The User PDA
      ownerWallet: a.string().required(), // The raw Public Key
      displayName: a.string(),
      bio: a.string(),
      avatarUrl: a.string(),
      bannerUrl: a.string(),
      website: a.string(),
      twitter: a.string(),
      // We store a cache of the stature for fast sorting/searching in the UI
      cachedStature: a.integer(), 
      isVerified: a.boolean().default(false),
    })
    .authorization((allow) => [
      allow.guest().to(['read']),
      allow.owner().to(['create', 'update', 'delete']),
    ]),

  // The 'id' here will be the Solana Company PDA string
  CompanyProfile: a
    .model({
      id: a.string().required(), // The Company PDA
      adminWallet: a.string().required(), // The admin Pubkey from Anchor
      name: a.string().required(),
      description: a.string(),
      logoUrl: a.string(),
      industry: a.string(),
      location: a.string(),
      isSuspended: a.boolean().default(false),
      // Weight and record cap cache for the Find page
      cachedWeight: a.integer(),
      cachedRecordCount: a.integer(),
    })
    .authorization((allow) => [
      allow.guest().to(['read']),
      allow.owner().to(['create', 'update', 'delete']),
    ]),

  // This bridges the on-chain StatureRecord with off-chain context
  WorkRecord: a
    .model({
      // ID can be: UserPDA + "_" + user_record_idx
      id: a.string().required(),
      userPda: a.string().required(),
      companyPda: a.string().required(),
      
      title: a.string().required(), // "Senior Smart Contract Audit"
      description: a.string(), // Long form details not fit for Solana
      attachmentUrl: a.string(), // Proof of work (PDF, Image, Github Link)
      
      onChainTimestamp: a.datetime().required(),
      statureChange: a.integer().required(), // The 'amount' i64 from Anchor
    })
    // Secondary indexes to quickly find all records for a user or company
    .secondaryIndexes((index) => [
      index('userPda').queryField('listRecordsByUser'),
      index('companyPda').queryField('listRecordsByCompany'),
    ])
    .authorization((allow) => [
      allow.guest().to(['read']),
      // Only the company that issued the record can edit the metadata
      allow.owner().to(['create', 'update']),
    ]),
});

export type Schema = ClientSchema<typeof schema>;

export const data = defineData({
  schema,
  authorizationModes: {
    // We use identityPool for guest access (Search) 
    // and userPool for the 'owner' logic (Claiming Identity)
    defaultAuthorizationMode: 'identityPool',
    apiKeyAuthorizationMode: {
      expiresInDays: 30,
    },
  },
});