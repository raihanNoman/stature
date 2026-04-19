"use client";

import { useState, useEffect, useCallback } from "react";
import { useWallet } from "../lib/wallet/context";
import { useSendTransaction } from "../lib/hooks/use-send-transaction";
import { toast } from "sonner";
import { address, type Address } from "@solana/kit";

import {
  getInitializeCompanyInstructionAsync,
  getInitializeUserInstructionAsync,
  getUpdateUserStatureInstruction,
} from "../generated/stature";

import { useCluster } from "./cluster-context";
import { parseTransactionError } from "../lib/errors";

export default function StatureCard() {
  const { wallet, signer, status } = useWallet();
  const { send, isSending } = useSendTransaction();
  const { getExplorerUrl } = useCluster();

  const [registeredProgramAddress, setCompanyAddress] = useState<Address | null>(null);
  const [userAddress, setUserAddress] = useState<Address | null>(null);

  const [companyName, setCompanyName] = useState("");
  const [website, setWebsite] = useState("");
  const [userName, setUserName] = useState("");
  const [honor, setHonor] = useState("");

  const walletAddress = wallet?.account.address;

  // 🔹 derive company PDA
  useEffect(() => {
    let cancelled = false;

    async function deriveCompany() {
      if (!signer) return;

      try {
        const ix = await getInitializeCompanyInstructionAsync({
          companyRep: signer,
          name: "temp",
        });

        const pda = ix.accounts[1]?.address;
        if (!cancelled) setCompanyAddress(pda as Address);
      } catch {
        if (!cancelled) setCompanyAddress(null);
      }
    }

    void deriveCompany();
    return () => {
      cancelled = true;
    };
  }, [signer]);

  // 🔹 derive user PDA
  useEffect(() => {
    let cancelled = false;

    async function deriveUser() {
      if (!signer) return;

      try {
        const ix = await getInitializeUserInstructionAsync({
          owner: signer,
          name: "temp",
        });

        const pda = ix.accounts[1]?.address;
        if (!cancelled) setUserAddress(pda as Address);
      } catch {
        if (!cancelled) setUserAddress(null);
      }
    }

    void deriveUser();
    return () => {
      cancelled = true;
    };
  }, [signer]);

  // ✅ Initialize Company
  const handleInitCompany = useCallback(async () => {
    if (!signer || !companyName || !website) return;

    try {
      const ix = await getInitializeCompanyInstructionAsync({
        companyRep: signer,
        name: companyName,
      });

      const sig = await send({ instructions: [ix] });

      toast.success("Company created!", {
        description: (
          <a href={getExplorerUrl(`/tx/${sig}`)} target="_blank">
            View transaction
          </a>
        ),
      });
    } catch (e) {
      const errorMessage = parseTransactionError(e);

      console.log(errorMessage, e);

      toast.error("Failed to initialize company" + errorMessage);
    }
  }, [signer, companyName, website, send, getExplorerUrl]);

  // ✅ Initialize User
  const handleInitUser = useCallback(async () => {
    if (!signer || !userName) return;

    try {
      const ix = await getInitializeUserInstructionAsync({
        owner: signer,
        name: userName,
      });

      const sig = await send({ instructions: [ix] });

      toast.success("User created!", {
        description: (
          <a href={getExplorerUrl(`/tx/${sig}`)} target="_blank">
            View transaction
          </a>
        ),
      });
    } catch (err) {
      console.error(err);
      toast.error("Failed to initialize user");
    }
  }, [signer, userName, send, getExplorerUrl]);

  // ✅ Update Honor
  const handleUpdateHonor = useCallback(async () => {
    if (!signer || !registeredProgramAddress || !userAddress || !honor) return;

    const recordAddres = address("temp address");
    const companyUserStateAddress = address("temp address"); // ⚠️ replace with real PDA if you have one

    console.warn(
      "this transaction will fail because the reocrd addresses are made u"
    );

    try {
      const ix = await getUpdateUserStatureInstruction({
        companyRep: signer,
        company: registeredProgramAddress,
        user: userAddress,
        record: recordAddres,
        amount: BigInt(honor),
        companyUserState: companyUserStateAddress,
        nonce: 1,
      });

      const sig = await send({ instructions: [ix] });

      toast.success("Honor updated!", {
        description: (
          <a href={getExplorerUrl(`/tx/${sig}`)} target="_blank">
            View transaction
          </a>
        ),
      });
    } catch (err) {
      console.error(err);
      toast.error("Failed to update honor");
    }
  }, [signer, registeredProgramAddress, userAddress, honor, send, getExplorerUrl]);

  if (status !== "connected") {
    return <div>Connect wallet</div>;
  }

  return (
    <div className="space-y-4 p-6 border rounded-xl">
      <h2 className="text-lg font-semibold">Stature System</h2>

      {/* Company */}
      <div className="space-y-2">
        <input
          placeholder="Company name"
          value={companyName}
          onChange={(e) => setCompanyName(e.target.value)}
        />
        <input
          placeholder="Website"
          value={website}
          onChange={(e) => setWebsite(e.target.value)}
        />
        <button onClick={handleInitCompany} disabled={isSending}>
          Create Company
        </button>
      </div>

      {/* User */}
      <div className="space-y-2">
        <input
          placeholder="User name"
          value={userName}
          onChange={(e) => setUserName(e.target.value)}
        />
        <button onClick={handleInitUser} disabled={isSending}>
          Create User
        </button>
      </div>

      {/* Honor */}
      <div className="space-y-2">
        <input
          placeholder="Honor amount"
          value={honor}
          onChange={(e) => setHonor(e.target.value)}
        />
        <button onClick={handleUpdateHonor} disabled={isSending}>
          Update Honor
        </button>
      </div>

      {/* Debug Info */}
      <div className="text-xs">
        <p>Company PDA: {registeredProgramAddress}</p>
        <p>User PDA: {userAddress}</p>
      </div>
    </div>
  );
}
