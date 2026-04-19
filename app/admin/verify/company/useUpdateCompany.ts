"use client";

import { useCluster } from "@/app/components/cluster-context";
import { getUpdateProgramRecordCapInstructionAsync, getUpdateProgramStatureInstructionAsync, getUpdateProgramSuspensionInstructionAsync, getUpdateProgramVerifiedStatusInstruction, getUpdateProgramVerifiedStatusInstructionAsync, getUpdateProgramWeightInstructionAsync } from "@/app/generated/stature";
import { parseTransactionError } from "@/app/lib/errors";
import { useSendTransaction } from "@/app/lib/hooks/use-send-transaction";
import { useWallet } from "@/app/lib/wallet/context";
import { address, isAddress } from "@solana/kit";
import { useState } from "react";
import { toast } from "sonner";
import type { Instruction } from "@solana/kit";

type Action =
  | { t: "verify"; value: true }
  | { t: "suspend"; value: boolean }
  | { t: "update-stature"; value: number }
  | { t: "update-weight"; value: number }
  | { t: "update-record-cap"; value: number };

export default function useUpdateCompany(companyPda: string | undefined) {
  const { signer } = useWallet();
  const { send, isSending } = useSendTransaction();
  const { getExplorerUrl } = useCluster();

  const [loading, setLoading] = useState(false);

  const updateCompany = async ( action: Action) => {
    try {
      setLoading(true);
      if (!signer) throw "no signer";
      else if (!companyPda) throw 'input a company pda first'
      else if (!isAddress(companyPda)) throw companyPda + "is not a company pda";

      const companuAddres = address(companyPda);

      let ix: Instruction | undefined = undefined;

      switch (action.t) {
        case "verify":
          ix = await getUpdateProgramVerifiedStatusInstructionAsync({
            admin: signer,
            targetProgram: companuAddres,
          });
          break;
        case "suspend":
          ix = await getUpdateProgramSuspensionInstructionAsync({
            admin: signer,
            targetProgram: companuAddres,
            'isSuspended': action.value, 
          });
          break;
        case "update-record-cap":
          ix = await getUpdateProgramRecordCapInstructionAsync({
            admin: signer,
            targetProgram: companuAddres,
            recordCap: BigInt(action.value),
          });
          break;
        case "update-stature":
          ix = await getUpdateProgramStatureInstructionAsync({
            admin: signer,
            targetProgram: companuAddres,
            newStature: BigInt(action.value),
          });
          break;
        case "update-weight":
          ix = await getUpdateProgramWeightInstructionAsync({
            admin: signer,
            targetProgram: companuAddres,
            weight: action.value,
          });
          break;
      }

      if (!ix) throw "no such instruction";

      const sig = await send({ instructions: [ix] });

      const txUrl = getExplorerUrl(`/tx/${sig}`);

      toast.success("Config created!" + txUrl);
      console.log("✅ success: verified company", txUrl);
    } catch (e) {
      const errorMessage = parseTransactionError(e);
      toast.error("Failed to verify company" + errorMessage);

      console.log("🚩 err verifying company", errorMessage, e);
    } finally {
      setLoading(false);
    }
  };

  return { updateCompany, loading: loading || isSending };
}
