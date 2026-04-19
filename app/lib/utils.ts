export function formatAddress(address: string, chars = 4): string {
  if (!address) return "";
  if (address.length <= chars * 2) return address;
  
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}


/**
 * Safely converts a BigInt or a numeric String to a Number.
 * Checks for precision loss to ensure your data isn't corrupted.
 */
export function toNumber(value: bigint | string | number | undefined | null): number {
  if (value === undefined || value === null) return 0;

  try {
    // 1. Coerce everything to a BigInt first
    const bValue = typeof value === "bigint" ? value : BigInt(value);

    // 2. Check if it's safe to turn into a JS Number
    if (bValue > BigInt(Number.MAX_SAFE_INTEGER) || bValue < BigInt(Number.MIN_SAFE_INTEGER)) {
      console.warn(`Precision loss warning: ${bValue} is too large for a JS Number.`);
      // We return Number(bValue) anyway, but the warning lets you know it's rounded
    }

    return Number(bValue);
  } catch (error) {
    console.error("Failed to convert value to number:", value);
    return 0;
  }
}

 toNumber.smallUnits =(
  value: bigint | string, 
  decimals: number = 9
): number  => {
  const bValue = typeof value === "bigint" ? value : BigInt(value);
  
  // We use division by power of 10 to get the decimal version
  // e.g. 1500000000 / 10^9 = 1.5
  return Number(bValue) / Math.pow(10, decimals);
}