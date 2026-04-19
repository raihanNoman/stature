// 1. REMOVE "use client" - This is a Server Component now
import React from "react";
import {
  ShieldCheck,
  Users,
  BarChart3,
  Zap,
  History,
  CheckCircle2,
  AlertCircle,
} from "lucide-react";

import { formatAddress, toNumber } from "@/app/lib/utils";
import { client } from "@/app/lib/aws";

// Initialize the client inside the server component or import a configured one

const MAX_CAP = 1000;

export default async function CompanyPage({
  params,
}: {
  params: Promise<{ pda: string }>;
}) {
  const { pda: programId } = await params;

  // 1. Fetch Data in Parallel for speed
  const [companyRes, companyUsersRes, workLogsRes] = await Promise.all([
    client.models.Program.get({ id: programId }),
    client.models.ProgramUser.usersByProgram(
      { programPda: programId },
      { selectionSet: ["user.*"] }
    ),
    client.models.StatureRecord.listRecordsByProgram({ programPda: programId }),
  ]);

  const company = companyRes.data;
  if (!company)
    return <div className="p-20 text-center">Company not found</div>;

  const companyUsers = companyUsersRes.data.map((m) => m.user);
  const workLogs = workLogsRes.data;

  // 2. Robust BigInt Math
  const totalPositive = BigInt(company.totalPositive || "0");
  const totalNegative = BigInt(company.totalNegative || "0");
  const recordCount = BigInt(company.recordCount || "0");

  let healthScore = 0;
  if (recordCount > 0n) {
    // Multiply by 100 first to keep precision in BigInt math
    healthScore = Number((totalPositive * 100n) / recordCount);
  }

  return (
    <div className="min-h-screen bg-zinc-50 text-zinc-900 dark:bg-black dark:text-zinc-100 p-4 md:p-8">
      <div className="max-w-6xl mx-auto">
        {/* Company Header */}
        <div className="flex flex-col md:flex-row justify-between items-start gap-8 mb-12">
          <div className="flex items-center gap-6">
            <div className="w-24 h-24 rounded-3xl bg-zinc-200 dark:bg-zinc-800 flex items-center justify-center border border-zinc-300 dark:border-zinc-700 shadow-xl">
              {company.logoUrl ? (
                <img
                  src={company.logoUrl}
                  alt="logo"
                  className="w-full h-full rounded-3xl object-cover"
                />
              ) : (
                <Zap size={40} className="text-yellow-500" />
              )}
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-4xl font-black tracking-tight uppercase">
                  {company.name}
                </h1>
                {company.isVerified && (
                  <ShieldCheck className="text-blue-500" size={24} />
                )}
              </div>
              <p className="font-mono text-zinc-500 text-sm mt-1">
                PDA: {formatAddress(company.id)}
              </p>
              <div className="flex gap-2 mt-3">
                <span className="px-2 py-1 rounded-md bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 text-[10px] font-bold uppercase tracking-widest">
                  {company.weight}x Weight
                </span>
                <span className="px-2 py-1 rounded-md bg-zinc-200 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 text-[10px] font-bold uppercase tracking-widest">
                  {toNumber(company.recordCount)} / {MAX_CAP} Records
                </span>
              </div>
            </div>
          </div>

          <div className="w-full md:w-auto p-6 rounded-3xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 flex flex-col items-center">
            <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest mb-1">
              Network Stature
            </span>
            <span className="text-5xl font-black tracking-tighter text-emerald-500">
              {company.stature || 0}
            </span>
          </div>
        </div>

        {/* Analytics Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          <MetricCard
            label="Reputation Integrity"
            value={`${healthScore}%`}
            icon={<BarChart3 size={20} />}
            subText={`${totalPositive.toString()} Pos vs ${totalNegative.toString()} Neg`}
          />
          <MetricCard
            label="Total Contributors"
            value={companyUsers.length.toString()}
            icon={<Users size={20} />}
            subText="Active verified members"
          />
          <MetricCard
            label="Capacity Used"
            value={`${Math.min(100, Math.round((toNumber(company.recordCount) / MAX_CAP) * 100))}%`}
            icon={<History size={20} />}
            subText={`${MAX_CAP - toNumber(company.recordCount)} mints left`}
          />
        </div>

        {/* Live Stature Feed */}
        <div className="rounded-3xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 overflow-hidden">
          <div className="p-6 border-b border-zinc-200 dark:border-zinc-800 flex justify-between items-center">
            <h2 className="font-bold flex items-center gap-2 uppercase tracking-tight">
              <History size={18} className="text-zinc-400" /> Recent Activity
            </h2>
          </div>

          <div className="divide-y divide-zinc-200 dark:divide-zinc-800">
            {workLogs.length > 0 ? (
              workLogs.map((log) => {
                const isPositive = BigInt(log.statureChange ?? 0n) >= 0n;

                return (
                  <div
                    key={log.signature}
                    className="p-4 flex items-center justify-between hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-colors"
                  >
                    <div className="flex items-center gap-4">
                      {isPositive ? (
                        <CheckCircle2 size={20} className="text-emerald-500" />
                      ) : (
                        <AlertCircle size={20} className="text-red-500" />
                      )}
                      <div>
                        <p className="font-bold text-sm">
                          {log.memo || "System Update"}
                        </p>
                        <p className="text-[10px] font-mono text-zinc-500">
                          {formatAddress(log.signature)}
                        </p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p
                        className={`font-black ${isPositive ? "text-emerald-500" : "text-red-500"}`}
                      >
                        {isPositive ? "+" : ""}
                        {log.statureChange}
                      </p>
                      <p className="text-[10px] text-zinc-400 uppercase">
                        {new Date(log.createdAt).toLocaleTimeString()}
                      </p>
                    </div>
                  </div>
                );
              })
            ) : (
              <div className="p-10 text-center text-zinc-500 text-sm">
                No recent activity found.
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// --- Helper Components ---
function MetricCard({
  label,
  value,
  icon,
  subText,
}: {
  label: string;
  value: string;
  icon: React.ReactNode;
  subText: string;
}) {
  return (
    <div className="p-6 rounded-3xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 shadow-sm">
      <div className="flex items-center gap-2 text-zinc-400 mb-4">
        {icon}
        <span className="text-[10px] font-bold uppercase tracking-widest">
          {label}
        </span>
      </div>
      <p className="text-3xl font-black mb-1">{value}</p>
      <p className="text-xs text-zinc-500">{subText}</p>
    </div>
  );
}
