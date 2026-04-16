"use client";

import React, { useState } from 'react';
import { Search, Building2, ShieldCheck, Clock, ExternalLink, Loader2 } from 'lucide-react';

export default function VerifyCompanies() {
  const [address, setAddress] = useState("");
  const [loading, setLoading] = useState(false);
  const [company, setCompany] = useState<any>(null);

  const handleSearch = () => {
    setLoading(true);
    // Mock Fetch from Amplify where isVerified == false
    setTimeout(() => {
      setCompany({
        name: "DevShop Alpha",
        pda: address || "Delta...92k1",
        admin: "Admin...77xQ",
        status: "probation"
      });
      setLoading(false);
    }, 800);
  };

  return (
    <div className="min-h-screen p-8 bg-zinc-50 dark:bg-black max-w-5xl mx-auto">
      <div className="flex justify-between items-end mb-12">
        <div>
          <h1 className="text-4xl font-black tracking-tighter uppercase">Verification Queue</h1>
          <p className="text-zinc-500 text-sm font-medium">Approve companies to issue Stature on-chain.</p>
        </div>
        <div className="flex gap-2 text-[10px] font-bold uppercase text-zinc-400 tracking-widest bg-zinc-200 dark:bg-zinc-900 px-4 py-2 rounded-full">
          <Clock size={12} /> 12 Pending Reviews
        </div>
      </div>

      {/* Search Bar */}
      <div className="relative mb-12">
        <input 
          type="text"
          placeholder="Enter Company PDA..."
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="w-full h-16 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl px-6 font-mono text-sm focus:ring-2 focus:ring-emerald-500 outline-none shadow-sm"
        />
        <button 
          onClick={handleSearch}
          className="absolute right-3 top-3 bottom-3 bg-black dark:bg-white text-white dark:text-black px-6 rounded-xl font-bold text-xs"
        >
          {loading ? <Loader2 className="animate-spin" /> : "SEARCH"}
        </button>
      </div>

      {/* Results Card */}
      {company && (
        <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-[2rem] overflow-hidden animate-in fade-in slide-in-from-bottom-4">
          <div className="p-8 flex flex-col md:flex-row justify-between items-center gap-6">
            <div className="flex items-center gap-6 text-center md:text-left">
              <div className="w-20 h-20 bg-zinc-100 dark:bg-zinc-800 rounded-3xl flex items-center justify-center">
                <Building2 size={32} className="text-zinc-400" />
              </div>
              <div>
                <h2 className="text-2xl font-black tracking-tight">{company.name}</h2>
                <p className="text-xs font-mono text-zinc-500 uppercase">{company.pda}</p>
                <a href="#" className="text-[10px] text-blue-500 font-bold hover:underline flex items-center gap-1 mt-1 uppercase">View Website <ExternalLink size={10} /></a>
              </div>
            </div>

            <div className="flex flex-col gap-3 w-full md:w-auto">
              <button className="h-12 bg-emerald-500 hover:bg-emerald-600 text-white rounded-xl px-10 font-black text-xs flex items-center justify-center gap-2 transition-all">
                <ShieldCheck size={16} /> VERIFY ON-CHAIN
              </button>
              <button className="text-[10px] font-bold text-red-500 uppercase hover:underline">Flag as Malicious</button>
            </div>
          </div>
          
          <div className="px-8 py-4 bg-zinc-50 dark:bg-zinc-800/30 border-t border-zinc-200 dark:border-zinc-800 text-[10px] text-zinc-400 font-bold uppercase tracking-widest">
            Admin Authority: {company.admin}
          </div>
        </div>
      )}
    </div>
  );
}