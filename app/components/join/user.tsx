"use client";

import React, { useState } from "react";
import { Loader2, UserPlus, Shield } from "lucide-react";
import { getCreateUserInstructionAsync } from "@/app/generated/stature";
import { useWallet } from "@/app/lib/wallet/context";
import { useSendTransaction } from "@/app/lib/hooks/use-send-transaction";
import { useCluster } from "../cluster-context";
import { toast } from "sonner";
import { client } from "@/app/lib/aws";
import { parseTransactionError } from "@/app/lib/errors";

export default function RegisterUserForm() {
    const { signer } = useWallet();
    const { send, isSending } = useSendTransaction();
    const { getExplorerUrl } = useCluster();

  const [loading, setLoading] = useState(false);
  const [name, setName] = useState("");

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      setLoading(true);
      if (!signer) throw "no signer";

      console.log("Registering user...", { name });

      // 1. SOLANA: Call Anchor instruction 'initialize_user'
      // const tx = await program.methods.initializeuser(name)...

      const ix = await getCreateUserInstructionAsync({
        'owner': signer,
        name,
      });
      const pda = ix.accounts[1]?.address;
      const sig = await send({ instructions: [ix] });
      const txUrl = getExplorerUrl(`/tx/${sig}`);

      toast.success("created user on solana!" + txUrl);
      console.log("✅ success: initialized user", txUrl);

      // 2. AMPLIFY: Store rich metadata using PDA as ID
      // await client.models.userProfile.create({ id: userPda, name, description })
      // later we will have an emit from solana -> amplify lamda -> have it saved in amplify

      const { data, errors } = await client.models.User.create({
       'wallet': signer.address, 
      });

      if (!data || errors) {
        console.log("err creating user on amplify", errors);
        throw "amplify err";
      }

      console.log("✅ Registration request submitted for verification!");
    } catch (e) {
                  const errorMessage = parseTransactionError(e);
      
      toast.error("Failed to initialize user" + errorMessage);
      console.log("🚩 err initializeing config", errorMessage, e);
    } finally {
      setLoading(false);
    }
  };


  return (
    <form onSubmit={handleRegister} className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 p-8 md:p-12 rounded-[2.5rem] shadow-2xl">
      <div className="space-y-8">
        <div>
          <label className="block text-[10px] font-bold uppercase tracking-widest text-zinc-400 mb-2">Display Name</label>
          <input 
            required
            type="text" 
            placeholder="What should the network call you?"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full bg-zinc-50 dark:bg-black border border-zinc-200 dark:border-zinc-800 rounded-xl px-5 py-4 font-bold outline-none focus:ring-2 focus:ring-blue-500 transition-all"
          />
        </div>

        <div className="rounded-2xl bg-blue-500/5 p-6 border border-blue-500/20">
          <div className="flex items-center gap-3 mb-2">
            <Shield size={18} className="text-blue-500" />
            <span className="text-xs font-bold uppercase tracking-wider">Privacy Notice</span>
          </div>
          <p className="text-xs text-zinc-500 italic">
            Your name and wallet address will be visible on the public Solana ledger. Your rich profile data (bio, avatar) will be stored on AWS Amplify.
          </p>
        </div>

        <button 
          disabled={loading}
          className="w-full h-16 bg-black dark:bg-white text-white dark:text-black rounded-2xl font-black uppercase tracking-widest text-sm flex items-center justify-center gap-2 hover:scale-[0.98] transition-all disabled:opacity-50"
        >
          {loading ? <Loader2 className="animate-spin" /> : <><UserPlus size={18} /> Initialize My Stature</>}
        </button>
      </div>
    </form>
  );
}