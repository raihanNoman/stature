"use client";

import React, { useState } from "react";
import {
  Building2,
  ShieldCheck,
  Loader2,
  Ban,
  TrendingUp,
  Scale,
  Database,
} from "lucide-react";
import { address, isAddress } from "@solana/kit";
import { fetchMaybeRegisteredProgram } from "@/app/generated/stature";
import { useSolanaClient } from "@/app/lib/solana-client-context";
import { client } from "@/app/lib/aws";
import { toast } from "sonner";
import useUpdateCompany from "./useUpdateCompany";
import { type Schema } from "@/amplify/data/resource";
import { parseTransactionError } from "@/app/lib/errors";

export default function VerifyCompanyPage() {
  const [loading, setLoading] = useState(false);
  const [program, setProgram] = useState<Schema["Program"]["type"]>();
  const [programAddressQuery, setProgramAddressQuery] = useState("");

  // Local state for admin inputs
  const [newStature, setNewStature] = useState("");
  const [newWeight, setNewWeight] = useState("");
  const [newCap, setNewCap] = useState("");

  const solanaClient = useSolanaClient();
  const { updateCompany, loading: isUpdating } = useUpdateCompany(program?.id);

  const handleLookup = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      if (!programAddressQuery) throw new Error("Enter an address");
      if (!isAddress(programAddressQuery))
        throw new Error("Invalid PDA format");

      const registeredProgramAddress = address(programAddressQuery);
      const companyAccount = await fetchMaybeRegisteredProgram(
        solanaClient.rpc,
        registeredProgramAddress
      );

      if (companyAccount.exists) {
        const ddbCompany = await client.models.Program.get({
          id: registeredProgramAddress,
        });

        if (ddbCompany.data) {
          setProgram(ddbCompany.data);
        } else {
          // Fallback sync logic... (your existing create logic)
          const { data } = await client.models.Program.create({
            id: companyAccount.address,
            name: companyAccount.data.name,
            isVerified: companyAccount.data.isVerified,
            isSuspended: companyAccount.data.isSuspended,
            recordCount: companyAccount.data.recordCount.toString(),
            stature: companyAccount.data.stature.toString(),
            totalNegative: companyAccount.data.totalNegativeTx.toString(),
            totalPositive: companyAccount.data.totalPositiveTx.toString(),
            weight: companyAccount.data.weight.toString(),
          });
          if (data) setProgram(data);
        }
        toast.success("Company synchronized");
      } else {
        toast.error("Account not found on-chain");
      }
    } catch (e) {
      toast.error(parseTransactionError(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen p-8 bg-zinc-50 dark:bg-black max-w-5xl mx-auto">
      {/* Header logic remains same */}
      <div className="flex justify-between items-end mb-12">
        <div>
          <h1 className="text-4xl font-black tracking-tighter uppercase">
            Admin Console
          </h1>
          <p className="text-zinc-500 text-sm font-medium">
            Manage on-chain company parameters.
          </p>
        </div>
      </div>

      {/* Search Bar remains same */}
      <div className="relative mb-12">
        <input
          type="text"
          placeholder="Enter Company PDA..."
          value={programAddressQuery}
          onChange={(e) => setProgramAddressQuery(e.target.value)}
          className="w-full h-16 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl px-6 font-mono text-sm focus:ring-2 focus:ring-emerald-500 outline-none"
        />
        <button
          onClick={handleLookup}
          className="absolute right-3 top-3 bottom-3 bg-black dark:bg-white text-white dark:text-black px-6 rounded-xl font-bold text-xs"
        >
          {loading ? <Loader2 className="animate-spin" size={16} /> : "LOOKUP"}
        </button>
      </div>

      {program && (
        <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-[2rem] overflow-hidden">
          {/* Top Info Section */}
          <div className="p-8 border-b border-zinc-100 dark:border-zinc-800 flex flex-col md:flex-row justify-between items-center gap-6">
            <div className="flex items-center gap-6">
              <div className="w-16 h-16 bg-zinc-100 dark:bg-zinc-800 rounded-2xl flex items-center justify-center">
                <Building2 size={24} className="text-zinc-400" />
              </div>
              <div>
                <h2 className="text-2xl font-black tracking-tight">
                  {program.name}
                </h2>
                <p className="text-xs font-mono text-zinc-400 uppercase">
                  {program.id}
                </p>
              </div>
            </div>

            <div className="flex gap-2">
              {!program.isVerified && (
                <button
                  onClick={() => updateCompany({ t: "verify", value: true })}
                  className="bg-emerald-500 hover:bg-emerald-600 text-white px-6 py-3 rounded-xl font-bold text-xs flex items-center gap-2"
                >
                  <ShieldCheck size={16} /> VERIFY
                </button>
              )}
              <button
                onClick={() =>
                  updateCompany({ t: "suspend", value: !program.isSuspended })
                }
                className={`${program.isSuspended ? "bg-zinc-200 text-black" : "bg-red-500 text-white"} px-6 py-3 rounded-xl font-bold text-xs flex items-center gap-2`}
              >
                <Ban size={16} />{" "}
                {program.isSuspended ? "UNSUSPEND" : "SUSPEND"}
              </button>
            </div>
          </div>

          {/* Management Controls Grid */}
          <div className="p-8 grid grid-cols-1 md:grid-cols-3 gap-6 bg-zinc-50/50 dark:bg-zinc-900/50">
            {/* Stature Control */}
            <AdminInput
              label="Stature Value"
              icon={<TrendingUp size={14} />}
              placeholder={program.stature || "0"}
              value={newStature}
              onChange={setNewStature}
              onAction={() =>
                updateCompany({
                  t: "update-stature",
                  value: Number(newStature),
                })
              }
            />

            {/* Weight Control */}
            <AdminInput
              label="Weight Multiplier"
              icon={<Scale size={14} />}
              placeholder={program.weight || "1"}
              value={newWeight}
              onChange={setNewWeight}
              onAction={() =>
                updateCompany({ t: "update-weight", value: Number(newWeight) })
              }
            />

            {/* Record Cap Control */}
            <AdminInput
              label="Record Cap"
              icon={<Database size={14} />}
              placeholder="1000"
              value={newCap}
              onChange={setNewCap}
              onAction={() =>
                updateCompany({ t: "update-record-cap", value: Number(newCap) })
              }
            />
          </div>

          <div className="px-8 py-4 bg-zinc-100 dark:bg-zinc-800/50 text-[10px] text-zinc-500 font-bold uppercase tracking-widest">
            Admin Authority Required for these actions
          </div>
        </div>
      )}
    </div>
  );
}

// Sub-component for Admin Inputs
function AdminInput({
  label,
  icon,
  value,
  onChange,
  onAction,
  placeholder,
}: any) {
  return (
    <div className="flex flex-col gap-2">
      <label className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest flex items-center gap-1">
        {icon} {label}
      </label>
      <div className="flex gap-2">
        <input
          type="number"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          className="flex-1 h-10 bg-white dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 rounded-lg px-3 text-sm outline-none focus:border-emerald-500"
        />
        <button
          onClick={onAction}
          className="h-10 px-4 bg-zinc-900 dark:bg-white text-white dark:text-black rounded-lg text-[10px] font-black hover:opacity-80 transition-opacity"
        >
          SET
        </button>
      </div>
    </div>
  );
}
