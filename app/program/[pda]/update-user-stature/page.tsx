"use client";

import React, { useState } from "react";
import {
  Search,
  UserCircle2,
  ArrowUpRight,
  TrendingUp,
  Loader2,
} from "lucide-react";
import { useWallet } from "@/app/lib/wallet/context";
import { useSendTransaction } from "@/app/lib/hooks/use-send-transaction";
import { useCluster } from "@/app/components/cluster-context";
import {
  fetchMaybeProgramUserState,
  fetchMaybeRegisteredProgram,
  fetchMaybeUser,
  findProgramUserStatePda,
  getUpdateUserStatureInstructionAsync,
} from "@/app/generated/stature";
import { address, Address, isAddress } from "@solana/kit";
import { toast } from "sonner";
import { findStatureRecordPda } from "./findStatureRecordPda";
import { useSolanaClient } from "@/app/lib/solana-client-context";
import { client } from "@/app/lib/aws";
import { Schema } from "@/amplify/data/resource";
import { parseTransactionError } from "@/app/lib/errors";

export default async function UpdateUserStature({
  params,
}: {
  params: Promise<{ pda: string }>;
}) {
  // the id is the pda of the company
  const { pda: companyPda } = await params;

  const solanaClient = useSolanaClient();

  const { signer } = useWallet();
  const { send, isSending } = useSendTransaction();
  const { getExplorerUrl } = useCluster();

  const [userAddressQuery, setUserAddressQuery] = useState("");
  const [amount, setAmount] = useState(0);
  const [loading, setLoading] = useState(false);
  const [user, setUser] = useState<Schema["User"]["type"]>();
  const [memo, setMemo] = useState("")

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      setLoading(true);
      if (!signer) throw "no signer";
      else if (!amount) throw "no amount selected";
      const amount_BigInt = BigInt(amount);

      if (!userAddressQuery) throw "no user address";
      else if (!isAddress(userAddressQuery))
        throw userAddressQuery + "is not a user pda";
      const userAddress = address(userAddressQuery);

      if (!companyPda) throw "no company pda";
      else if (!isAddress(companyPda)) throw companyPda + "is not a user pda";

      const registeredProgramAddress = address(companyPda); // ! need to do a check to make sure its actually an address

      console.log("Updaing user stature...", {
        user: userAddress,
        company: companyPda,
        amount: amount_BigInt,
      });

      // 1. SOLANA: Call Anchor instruction 'initialize_company'
      // const tx = await program.methods.initializeCompany(name)...

      const registeredProgramAccount = await fetchMaybeRegisteredProgram(
        solanaClient.rpc,
        registeredProgramAddress
      );

      const userAccount = await fetchMaybeUser(
        solanaClient.rpc,
        signer.address
      );
      if (!userAccount.exists) throw "no such user account";

      const [recordAddress] = await findStatureRecordPda({
        user: userAccount.address,
        userRecordIdx: userAccount.data.recordIdx,
      });

      const [stateAddress] = await findProgramUserStatePda({
        registeredProgram: registeredProgramAccount.address,
        userWallet: userAccount.address,
      });

      let nonce = 0n;
      try {
        const state = await fetchMaybeProgramUserState(
          solanaClient.rpc,
          stateAddress
        );

        // if (state.exists) nonce = state.data.;
        // else console.log("New user-company relationship.", nonce);
      } catch (e) {
        console.log("err getting company-user-state pda", e);
      }

      const ix = await getUpdateUserStatureInstructionAsync({
        stature: amount_BigInt,
        'payer': signer, 
        'userWallet': userAddress, 
        'targetProgram': registeredProgramAccount.address, 
        'registeredProgramSourceAccount': registeredProgramAccount.address, 
        'memo' : memo, 
        record: recordAddress,
        user: userAddress,
      });

      const sig = await send({ instructions: [ix] });
      const txUrl = getExplorerUrl(`/tx/${sig}`);

      toast.success("created company on solana!" + txUrl);
      console.log("✅ success: initialized company", txUrl);

      // 2. AMPLIFY: Store rich metadata using PDA as ID
      // await client.models.CompanyProfile.create({ id: companyPda, name, description })
      // later we will have an emit from solana -> amplify lamda -> have it saved in amplify

      const user = await client.models.User.get({
        wallet: userAddressQuery,
      });
      const newStature = (userAccount.data.stature + amount_BigInt).toString();

      if (user.data) {
        const { data, errors } = await client.models.User.update({
          wallet: userAccount.address,
          stature: newStature.toString(), // get new amount
        });

        if (!data || errors) {
          console.log("err updating user stature on amplify", errors);
          throw "amplify err";
        }
      } else {
        console.log("no existing user on dynanoDB. creating new user....");
        const { data, errors } = await client.models.User.create({
          wallet: userAccount.address,
          stature: newStature.toString(), // get new amount
        });

        if (!data || errors) {
          console.log("err creating user stature on amplify", errors);
          throw "amplify err";
        }
      }

      // if there is no such user you gotta create that user.

      console.log("✅ Registration request submitted for verification!");
    } catch (e) {
      const errorMessage = parseTransactionError(e);

      toast.error("Failed to update user stature" + errorMessage);
      console.log("🚩 err updating user stature", errorMessage, e);
    } finally {
      setLoading(false);
    }
  };

  const handleLookup = async (e: React.FormEvent) => {
    e.preventDefault();

    try {
      setLoading(true);

      if (!userAddressQuery) throw "no user address";
      const userAddress = address(userAddressQuery);

      const userAccount = await fetchMaybeUser(solanaClient.rpc, userAddress);
      if (userAccount.exists) {
        console.log("solana user found!");

        const user = await client.models.User.get({
          wallet: userAddress,
        });

        if (user.data) {
          console.log("dynamoDB user found!");
          setUser(user.data);
        } else {
          console.log(
            "no existing user on dynanoDB. fallback creating new user...."
          );

          const { data, errors } = await client.models.User.create({
            wallet: userAccount.address,
          });

          if (!data || errors) {
            console.log("err creating user stature on amplify", errors);
            throw "amplify err";
          }
        }
      } else {
        console.log("no such solana user ...lets initiate one");
      }

      console.log("✅ Registration request submitted for verification!");
    } catch (e) {
      toast.error("Failed to look up user");
      console.log("🚩 err looking up user", e);
    } finally {
      setLoading(false);
    }
  };
  return (
    <div className="min-h-screen p-8 bg-zinc-50 dark:bg-[#050505] max-w-4xl mx-auto">
      <div className="mb-12">
        <h1 className="text-4xl font-black tracking-tighter uppercase">
          Issue Stature
        </h1>
        <p className="text-zinc-500 text-sm italic">
          Assign proof-of-work credit to a contributor's ledger.
        </p>
      </div>

      {/* Lookup Bar */}
      <div className="flex gap-4 mb-12">
        <div className="relative flex-1">
          <Search
            className="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-400"
            size={20}
          />
          <input
            type="text"
            placeholder="User Wallet or PDA..."
            value={userAddressQuery}
            onChange={(e) => setUserAddressQuery(e.target.value)}
            className="w-full h-14 pl-12 pr-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl font-mono text-sm outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <button
          onClick={handleLookup}
          className="bg-black dark:bg-white text-white dark:text-black px-8 rounded-xl font-black text-xs uppercase"
        >
          {loading ? <Loader2 className="animate-spin" /> : "Lookup"}
        </button>
      </div>

      {userAddressQuery && user ? (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 animate-in fade-in zoom-in-95 duration-300">
          {/* User Card */}
          <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 p-8 rounded-[2rem] shadow-xl">
            <div className="flex items-center gap-4 mb-8">
              <UserCircle2 size={48} className="text-zinc-300" />
              <div>
                <h3 className="font-black text-xl leading-none">{user.name}</h3>
                <p className="text-[10px] font-mono text-zinc-500 mt-1">
                  {user.wallet}
                </p>
              </div>
            </div>

            <div className="space-y-4">
              <StatRow
                label="Global Stature"
                value={user.stature}
                icon={<TrendingUp size={14} />}
              />
            </div>
          </div>

          {/* Update Form */}
          <div className="bg-black text-white p-8 rounded-[2rem] shadow-2xl flex flex-col justify-between">
            <div>
              <span className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">
                Update Amount
              </span>
              <div className="flex items-center gap-4 mt-4 mb-8">
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(Number(e.target.value))}
                  className="bg-transparent text-5xl font-black w-full outline-none text-emerald-500"
                />
                <div className="bg-emerald-500/10 text-emerald-500 p-3 rounded-2xl">
                  <ArrowUpRight size={32} />
                </div>
              </div>
            </div>

            <button
              onClick={handleRegister}
              className="w-full h-14 bg-white text-black rounded-xl font-black uppercase tracking-widest text-xs hover:scale-[0.98] transition-all"
            >
              Commit to Ledger
            </button>
          </div>
        </div>
      ) : userAddressQuery ? (
        <div>
          <div className="bg-black text-white p-8 rounded-[2rem] shadow-2xl flex flex-col justify-between">
            <div>
              <span className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">
                Create And Assign Amount
              </span>
              <div className="flex items-center gap-4 mt-4 mb-8">
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(Number(e.target.value))}
                  className="bg-transparent text-5xl font-black w-full outline-none text-emerald-500"
                />
                <div className="bg-emerald-500/10 text-emerald-500 p-3 rounded-2xl">
                  <ArrowUpRight size={32} />
                </div>
              </div>
            </div>

            <button
              onClick={handleRegister}
              className="w-full h-14 bg-white text-black rounded-xl font-black uppercase tracking-widest text-xs hover:scale-[0.98] transition-all"
            >
              Commit to Ledger
            </button>
          </div>
        </div>
      ) : null}
    </div>
  );
}

function StatRow({ label, value, icon }: any) {
  return (
    <div className="flex justify-between items-center p-3 bg-zinc-50 dark:bg-zinc-800/50 rounded-xl">
      <div className="flex items-center gap-2 text-zinc-500">
        {icon}
        <span className="text-[10px] font-bold uppercase tracking-widest">
          {label}
        </span>
      </div>
      <span className="font-black">{value}</span>
    </div>
  );
}
