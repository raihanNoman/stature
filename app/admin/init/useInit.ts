"use client";

import { useCluster } from "@/app/components/cluster-context";
import { getCreateAdminInstructionAsync } from "@/app/generated/stature";
import { parseTransactionError } from "@/app/lib/errors";
import { useSendTransaction } from "@/app/lib/hooks/use-send-transaction";
import { useWallet } from "@/app/lib/wallet/context";
import { useState } from "react";
import { toast } from "sonner";

export default function useInit() {
  const { signer } = useWallet();
  const { send, isSending } = useSendTransaction();
  const { getExplorerUrl } = useCluster();

  const [loading, setLoading] = useState(false);

  const handleInitialize = async () => {
    try {
      setLoading(true);
      if (!signer) throw "no signer";

      const ix = await getCreateAdminInstructionAsync({
        admin: signer,
      });

      const sig = await send({ instructions: [ix] });

      const txUrl = getExplorerUrl(`/tx/${sig}`);

      toast.success("Config created!" + txUrl);
      console.log("✅ success: initialized config", txUrl);
    } catch (e) {
      const errorMessage = parseTransactionError(e);
      toast.error("Failed to initialize user" + errorMessage);
      console.log("🚩 err initializeing config", errorMessage, e);
    } finally {
      setLoading(false);
    }
  };

  return { handleInitialize, loading: loading || isSending };
}
