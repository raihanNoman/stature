"use client";

import React, { useState } from 'react';
import { Search, UserCircle2, ArrowUpRight, TrendingUp, History, Loader2 } from 'lucide-react';

export default function UpdateUserStature() {
  const [query, setQuery] = useState("");
  const [amount, setAmount] = useState(0);
  const [loading, setLoading] = useState(false);
  const [user, setUser] = useState<any>(null);

  const handleLookup = () => {
    setLoading(true);
    // Simulate lookup
    setTimeout(() => {
      setUser({ name: "Sarwar Engineer", totalStature: 1450, fromThisCompany: 250, address: "7xWp...3u9Y" });
      setLoading(false);
    }, 1000);
  };

  return (
    <div className="min-h-screen p-8 bg-zinc-50 dark:bg-[#050505] max-w-4xl mx-auto">
      <div className="mb-12">
        <h1 className="text-4xl font-black tracking-tighter uppercase">Issue Stature</h1>
        <p className="text-zinc-500 text-sm italic">Assign proof-of-work credit to a contributor's ledger.</p>
      </div>

      {/* Lookup Bar */}
      <div className="flex gap-4 mb-12">
        <div className="relative flex-1">
          <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-400" size={20} />
          <input 
            type="text"
            placeholder="User Wallet or PDA..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
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

      {user && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 animate-in fade-in zoom-in-95 duration-300">
          
          {/* User Card */}
          <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 p-8 rounded-[2rem] shadow-xl">
            <div className="flex items-center gap-4 mb-8">
              <UserCircle2 size={48} className="text-zinc-300" />
              <div>
                <h3 className="font-black text-xl leading-none">{user.name}</h3>
                <p className="text-[10px] font-mono text-zinc-500 mt-1">{user.address}</p>
              </div>
            </div>

            <div className="space-y-4">
               <StatRow label="Global Stature" value={user.totalStature} icon={<TrendingUp size={14}/>} />
               <StatRow label="Contributions Here" value={user.fromThisCompany} icon={<History size={14}/>} />
            </div>
          </div>

          {/* Update Form */}
          <div className="bg-black text-white p-8 rounded-[2rem] shadow-2xl flex flex-col justify-between">
            <div>
              <span className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">Update Amount</span>
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

            <button className="w-full h-14 bg-white text-black rounded-xl font-black uppercase tracking-widest text-xs hover:scale-[0.98] transition-all">
              Commit to Ledger
            </button>
          </div>

        </div>
      )}
    </div>
  );
}

function StatRow({ label, value, icon }: any) {
  return (
    <div className="flex justify-between items-center p-3 bg-zinc-50 dark:bg-zinc-800/50 rounded-xl">
      <div className="flex items-center gap-2 text-zinc-500">
        {icon}
        <span className="text-[10px] font-bold uppercase tracking-widest">{label}</span>
      </div>
      <span className="font-black">{value}</span>
    </div>
  );
}