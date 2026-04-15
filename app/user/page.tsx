import React from 'react';
import { ShieldCheck, Briefcase, ExternalLink, Award, Search } from 'lucide-react';

// --- Types ---
interface StatureRecordUI {
  id: string;
  companyName: string;
  companyImage: string;
  amount: number; // maps to i64 stature change
  timestamp: number;
  description: string; // From Amplify
}

export default function UserPage() {
  // Dummy Data - In production, fetch from Solana (User account) + Amplify (Metadata)
  const userData = {
    name: "S. Engineer",
    address: "7xWp...3u9Y",
    stature: 1250,
    recordCount: 14,
    isVerified: true
  };

  const records: StatureRecordUI[] = [
    {
      id: "1",
      companyName: "DevShop Delta",
      companyImage: "https://api.dicebear.com/7.x/identicon/svg?seed=delta",
      amount: 50,
      timestamp: Date.now() - 86400000,
      description: "Completed Smart Contract Audit for Stature Protocol"
    },
    {
      id: "2",
      companyName: "Takaful Club",
      companyImage: "https://api.dicebear.com/7.x/identicon/svg?seed=takaful",
      amount: -10,
      timestamp: Date.now() - 500000000,
      description: "Missed claim verification deadline"
    }
  ];

  return (
    <div className="min-h-screen bg-zinc-50 text-zinc-900 dark:bg-black dark:text-zinc-100 p-4 md:p-8">
      <div className="max-w-4xl mx-auto">
        
        {/* Profile Header */}
        <div className="flex flex-col md:flex-row items-center gap-6 mb-12">
          <div className="relative">
            <div className="w-32 h-32 rounded-full bg-gradient-to-tr from-emerald-500 to-blue-600 p-1">
              <div className="w-full h-full rounded-full bg-white dark:bg-black flex items-center justify-center overflow-hidden">
                <img 
                  src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${userData.name}`} 
                  alt="Profile" 
                />
              </div>
            </div>
            {userData.isVerified && (
              <div className="absolute bottom-1 right-1 bg-blue-500 p-1.5 rounded-full border-2 border-white dark:border-black">
                <ShieldCheck size={16} className="text-white" />
              </div>
            )}
          </div>

          <div className="text-center md:text-left flex-1">
            <h1 className="text-3xl font-black tracking-tight uppercase italic">
              {userData.name}
            </h1>
            <p className="font-mono text-zinc-500 text-sm mt-1">{userData.address}</p>
            <div className="flex gap-4 mt-4 justify-center md:justify-start">
              <StatBlock label="Stature Score" value={userData.stature.toLocaleString()} highlight />
              <StatBlock label="Verified Records" value={userData.recordCount} />
            </div>
          </div>
        </div>

        {/* The Reputation Ledger (Work History) */}
        <section>
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-xl font-bold flex items-center gap-2">
              <Award className="text-emerald-500" /> Soulbound Ledger
            </h2>
            <button className="text-xs font-mono text-zinc-500 hover:text-emerald-500 transition-colors">
              VIEW ON SOLSCAN ↗
            </button>
          </div>

          <div className="space-y-4">
            {records.map((record) => (
              <RecordCard key={record.id} record={record} />
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}

// --- Sub-components for Cleanliness ---

function StatBlock({ label, value, highlight = false }: { label: string, value: string | number, highlight?: boolean }) {
  return (
    <div className={`px-4 py-2 rounded-2xl border ${highlight ? 'bg-black text-white dark:bg-zinc-100 dark:text-black border-transparent' : 'border-zinc-200 dark:border-zinc-800'}`}>
      <p className="text-[10px] uppercase font-bold tracking-widest opacity-60">{label}</p>
      <p className="text-xl font-black tracking-tighter">{value}</p>
    </div>
  );
}

function RecordCard({ record }: { record: StatureRecordUI }) {
  const isPositive = record.amount > 0;
  
  return (
    <div className="group relative overflow-hidden rounded-3xl border border-zinc-200 bg-white p-6 transition-all hover:shadow-xl dark:border-zinc-800 dark:bg-zinc-900/50">
      <div className="flex items-start justify-between gap-4">
        <div className="flex gap-4">
          <img src={record.companyImage} alt="" className="h-12 w-12 rounded-xl" />
          <div>
            <h3 className="font-bold leading-none">{record.companyName}</h3>
            <p className="mt-2 text-sm text-zinc-600 dark:text-zinc-400">
              {record.description}
            </p>
            <p className="mt-2 text-[10px] font-mono text-zinc-400 uppercase">
              {new Date(record.timestamp).toLocaleDateString()}
            </p>
          </div>
        </div>
        
        <div className={`text-right font-black text-lg ${isPositive ? 'text-emerald-500' : 'text-red-500'}`}>
          {isPositive ? '+' : ''}{record.amount}
        </div>
      </div>
    </div>
  );
}