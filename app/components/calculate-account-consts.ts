import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { PythHttpClient, getPythProgramKeyForCluster } from '@pythnetwork/client';

const USER_STATURE_SIZE = 108; // Update this to match your actual Rust struct size
const PYTH_SOL_PRICE_FEED = new PublicKey("H6ARHf6YX79fsS96C9TUnP8fE8uCjS9GCo57y8p7h9Ea"); // Mainnet-Beta

export async function calculateStudentRunway(connection: Connection, walletAddress: PublicKey) {
    // 1. Get Wallet Balance
    const balance = await connection.getBalance(walletAddress);
    const solBalance = balance / LAMPORTS_PER_SOL;

    // 2. Get Rent Cost per Student
    const lamportsPerStudent = await connection.getMinimumBalanceForRentExemption(USER_STATURE_SIZE);
    
    // Add transaction fee overhead (standard 5,000 lamports per update)
    const totalCostPerStudent = lamportsPerStudent + 5000;
    const solPerStudent = totalCostPerStudent / LAMPORTS_PER_SOL;

    // 3. Get SOL Price from Pyth (Free/Public Feed)
    const pythClient = new PythHttpClient(connection, getPythProgramKeyForCluster('mainnet-beta'));
    const data = await pythClient.getData();
    const solPrice = data.productPrice.get('Crypto.SOL/USD')?.price || 0;

    // 4. Calculate Capacity
    const studentCapacity = Math.floor(balance / totalCostPerStudent);
    const usdValue = solBalance * solPrice;

    return {
        currentSol: solBalance.toFixed(4),
        currentUsd: `$${usdValue.toFixed(2)}`,
        costPerStudentSol: solPerStudent.toFixed(6),
        affordableStudents: studentCapacity,
        runwayStatus: studentCapacity < 50 ? "⚠️ LOW FUNDS" : "✅ HEALTHY"
    };
}

// Example usage:
// const runway = await calculateStudentRunway(connection, myKeypair.publicKey);
// console.log(`You can afford ${runway.affordableStudents} more students.`);