import React from "react";
import { ShieldCheck, Briefcase, ExternalLink, Award, Zap } from "lucide-react";
import { type Schema } from "@/amplify/data/resource";
import { formatAddress } from "@/app/lib/utils";
import { client } from "@/app/lib/aws";

export default async function UserPage({
  params,
}: {
  params: Promise<{ pda: string }>;
}) {
  const { pda: userWallet } = await params;

  // 1. Parallel Data Fetching
  const [userRes, membershipsRes, workLogsRes] = await Promise.all([
    client.models.User.get({ wallet: userWallet }),
    client.models.ProgramUser.programsByUser(
      { userWallet: userWallet },
      { selectionSet: ["program.*"] }
    ),
    client.models.StatureRecord.listRecordsByUser({ userWallet: userWallet }), // Assuming index on userId
  ]);

  const user = userRes.data;
  if (!user)
    return <div className="p-20 text-center font-black">USER NOT FOUND</div>;

  const memberships = membershipsRes.data;
  const records = workLogsRes.data;

  return (
    <div className="min-h-screen bg-zinc-50 text-zinc-900 dark:bg-black dark:text-zinc-100 p-4 md:p-8">
      <div className="max-w-4xl mx-auto">
        {/* Profile Header */}
        <div className="flex flex-col md:flex-row items-center gap-6 mb-12">
          <div className="relative">
            <div className="w-32 h-32 rounded-full bg-gradient-to-tr from-emerald-500 to-blue-600 p-1">
              <div className="w-full h-full rounded-full bg-white dark:bg-black flex items-center justify-center overflow-hidden">
                {user.avatarUrl ? (
                  <img
                    src={user.avatarUrl}
                    alt="Profile"
                    className="w-full h-full object-cover"
                  />
                ) : (
                  <img
                    src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.name || user.wallet}`}
                    alt="Profile"
                  />
                )}
              </div>
            </div>
            {user.isVerified && (
              <div className="absolute bottom-1 right-1 bg-blue-500 p-1.5 rounded-full border-2 border-white dark:border-black">
                <ShieldCheck size={16} className="text-white" />
              </div>
            )}
          </div>

          <div className="text-center md:text-left flex-1">
            <h1 className="text-3xl font-black tracking-tight uppercase italic">
              {user.name || "Anonymous User"}
            </h1>
            <p className="font-mono text-zinc-500 text-sm mt-1">
              {formatAddress(user.wallet)}
            </p>
            <div className="flex gap-4 mt-4 justify-center md:justify-start">
              <StatBlock
                label="Global Stature"
                value={user.stature || "0"}
                highlight
              />
              <StatBlock label="Network Nodes" value={memberships.length} />
            </div>
          </div>
        </div>

        {/* Memberships/Affiliations */}
        {memberships.length > 0 && (
          <section className="mb-12">
            <h2 className="text-[10px] font-bold text-zinc-400 uppercase tracking-[0.2em] mb-4">
              Active Affiliations
            </h2>
            <div className="flex flex-wrap gap-3">
              {memberships.map((m) => (
                <div
                  key={m.program.id}
                  className="flex items-center gap-2 px-3 py-2 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl"
                >
                  <div className="w-5 h-5 rounded-md bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center overflow-hidden">
                    {m.program.logoUrl ? (
                      <img src={m.program.logoUrl} className="object-cover" />
                    ) : (
                      <Zap size={10} />
                    )}
                  </div>
                  <span className="text-xs font-bold">{m.program.name}</span>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* The Reputation Ledger */}
        <section>
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-xl font-bold flex items-center gap-2 uppercase tracking-tighter">
              <Award className="text-emerald-500" /> Soulbound Ledger
            </h2>
            <a
              href={`https://solscan.io/account/${user.wallet}`}
              target="_blank"
              className="text-[10px] font-mono text-zinc-500 hover:text-emerald-500 transition-colors flex items-center gap-1"
            >
              VERIFY ON-CHAIN <ExternalLink size={10} />
            </a>
          </div>

          <div className="space-y-4">
            {records.length > 0 ? (
              records.map((record) => (
                <RecordCard key={record.signature} record={record} />
              ))
            ) : (
              <div className="p-12 text-center border-2 border-dashed border-zinc-200 dark:border-zinc-800 rounded-3xl">
                <p className="text-zinc-500 text-sm">
                  No stature records minted yet.
                </p>
              </div>
            )}
          </div>
        </section>
      </div>
    </div>
  );
}

// --- Sub-components ---

function StatBlock({
  label,
  value,
  highlight = false,
}: {
  label: string;
  value: string | number;
  highlight?: boolean;
}) {
  return (
    <div
      className={`px-5 py-3 rounded-2xl border ${highlight ? "bg-zinc-900 text-white dark:bg-zinc-100 dark:text-black border-transparent" : "border-zinc-200 dark:border-zinc-800"}`}
    >
      <p className="text-[9px] uppercase font-black tracking-widest opacity-60 mb-0.5">
        {label}
      </p>
      <p className="text-2xl font-black tracking-tighter leading-none">
        {value}
      </p>
    </div>
  );
}

function RecordCard({ record }: { record: Schema["StatureRecord"]["type"] }) {
  const isPositive = BigInt(record.statureChange ?? 0n) >= 0n;

  return (
    <div className="group relative overflow-hidden rounded-3xl border border-zinc-200 bg-white p-6 transition-all hover:shadow-lg dark:border-zinc-800 dark:bg-zinc-900/40">
      <div className="flex items-start justify-between gap-4">
        <div className="flex gap-4">
          <div className="h-12 w-12 rounded-2xl bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center border border-zinc-200 dark:border-zinc-700">
            <Briefcase size={20} className="text-zinc-400" />
          </div>
          <div>
            <p className="mt-2 text-sm text-zinc-600 dark:text-zinc-400 line-clamp-2">
              {record.memo}
            </p>
            <div className="flex items-center gap-3 mt-3">
              <span className="text-[10px] font-mono text-zinc-400 bg-zinc-100 dark:bg-zinc-800 px-2 py-0.5 rounded">
                TX: {formatAddress(record.signature)}
              </span>
              <span className="text-[10px] text-zinc-400 uppercase font-bold">
                {new Date(record.createdAt).toLocaleDateString()}
              </span>
            </div>
          </div>
        </div>

        <div
          className={`text-right font-black text-xl ${isPositive ? "text-emerald-500" : "text-red-500"}`}
        >
          {isPositive ? "+" : ""}
          {record.statureChange}
        </div>
      </div>
    </div>
  );
}
