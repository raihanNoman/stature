import {
  isSolanaError,
  SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM,
} from "@solana/kit";
import {
  getStatureErrorMessage,
  STATURE_ERROR__ADMIN_ACTION_ONLY,
  STATURE_ERROR__ALREADY_INITIALIZED,
  STATURE_ERROR__CANNOT_SELF_ASSIGN_STATURE,
  STATURE_ERROR__PROGRAM_NOT_VERIFIED,
  STATURE_ERROR__PROGRAM_SUSPENDED,
  STATURE_ERROR__INVALID_NONCE,
  STATURE_ERROR__INVALID_RECORD,
  STATURE_ERROR__RATE_LIMITED,
  STATURE_ERROR__INVALID_WEIGHT,
  STATURE_ERROR__OVERFLOW,
  STATURE_ERROR__REQUEST_LIMIT_INCREASE,
  STATURE_ERROR__STRING_TOO_LONG,
  STATURE_ERROR__TOO_MANY_UPDATES,
  STATURE_ERROR__TOO_MUCH,
  STATURE_ERROR__USER_SUSPENDED,
  // Removed old Vault imports and ensured Stature types are present
  type StatureError,
} from "../generated/stature";

/**
 * Mapping of numeric error codes to StatureError types.
 * This allows the parser to lookup the code returned by the SVM
 * and match it to a human-readable message via getStatureErrorMessage.
 */
const STATURE_ERROR_CODES: Record<number, StatureError> = {
  [STATURE_ERROR__ADMIN_ACTION_ONLY]: STATURE_ERROR__ADMIN_ACTION_ONLY,
  [STATURE_ERROR__ALREADY_INITIALIZED]: STATURE_ERROR__ALREADY_INITIALIZED,
  [STATURE_ERROR__CANNOT_SELF_ASSIGN_STATURE]:
    STATURE_ERROR__CANNOT_SELF_ASSIGN_STATURE,
  [STATURE_ERROR__PROGRAM_NOT_VERIFIED]: STATURE_ERROR__PROGRAM_NOT_VERIFIED,
  [STATURE_ERROR__PROGRAM_SUSPENDED]: STATURE_ERROR__PROGRAM_SUSPENDED,
  [STATURE_ERROR__INVALID_NONCE]: STATURE_ERROR__INVALID_NONCE,
  [STATURE_ERROR__INVALID_RECORD]: STATURE_ERROR__INVALID_RECORD,
  [STATURE_ERROR__RATE_LIMITED]: STATURE_ERROR__RATE_LIMITED,
  [STATURE_ERROR__INVALID_WEIGHT]: STATURE_ERROR__INVALID_WEIGHT,
  [STATURE_ERROR__OVERFLOW]: STATURE_ERROR__OVERFLOW,
  [STATURE_ERROR__REQUEST_LIMIT_INCREASE]:
    STATURE_ERROR__REQUEST_LIMIT_INCREASE,
  [STATURE_ERROR__STRING_TOO_LONG]: STATURE_ERROR__STRING_TOO_LONG,
  [STATURE_ERROR__TOO_MANY_UPDATES]: STATURE_ERROR__TOO_MANY_UPDATES,
  [STATURE_ERROR__TOO_MUCH]: STATURE_ERROR__TOO_MUCH,
  [STATURE_ERROR__USER_SUSPENDED]: STATURE_ERROR__USER_SUSPENDED,
};

export function parseTransactionError(err: unknown): string {
  // 1. Handle UI/Wallet rejections
  if (err instanceof Error && err.message.includes("User rejected")) {
    return "Transaction was rejected by the wallet.";
  }

  // 2. Handle Stature Program Custom Errors
  if (
    isSolanaError(err, SOLANA_ERROR__INSTRUCTION_ERROR__CUSTOM) &&
    typeof err.context?.code === "number"
  ) {
    const statureError = STATURE_ERROR_CODES[err.context.code];
    if (statureError !== undefined) {
      return getStatureErrorMessage(statureError);
    }
  }

  // 3. Fallback to deepest message in the cause chain
  const message = getDeepestMessage(err);

  // Truncate overly long system errors for cleaner UI toasts
  return message.length > 200 ? `${message.slice(0, 200)}...` : message;
}

function getDeepestMessage(err: unknown): string {
  let deepest = err instanceof Error ? err.message : String(err);
  let current: unknown = err;

  while (current instanceof Error && current.cause) {
    current = current.cause;
    if (current instanceof Error) {
      deepest = current.message;
    }
  }

  return deepest;
}
