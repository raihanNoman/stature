/**
 * This code was manually added to follow the Codama pattern.
 * This helper derives the PDA for a specific Stature Record.
 */

import { STATURE_PROGRAM_ADDRESS } from "@/app/generated/stature";
import {
  getAddressEncoder,
  getBytesEncoder,
  getU64Encoder, // Needed for the u64 index
  getProgramDerivedAddress,
  type Address,
  type ProgramDerivedAddress,
} from "@solana/kit";

export type StatureRecordSeeds = {
  /** The public key of the user this record belongs to */
  user: Address;
  /** The specific record index (user.record_idx) */
  userRecordIdx: number | bigint;
};

export async function findStatureRecordPda(
  seeds: StatureRecordSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const { programAddress = STATURE_PROGRAM_ADDRESS } = config;

  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      // "stature_record" string as Uint8Array
      getBytesEncoder().encode(
        new Uint8Array([
          115, 116, 97, 116, 117, 114, 101, 95, 114, 101, 99, 111, 114, 100,
        ])
      ),
      // User Public Key
      getAddressEncoder().encode(seeds.user),
      // User Record Index encoded as u64 Little-Endian (Anchor default)
      getU64Encoder().encode(BigInt(seeds.userRecordIdx)),
    ],
  });
}
