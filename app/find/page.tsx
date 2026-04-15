"use client"
import React, { useState } from 'react';
import { Search, Shield, Building2, User, ArrowRight, Fingerprint } from 'lucide-react';

export default function FindPage() {
  const [query, setQuery] = useState('');
  
  // Dummy logic: In production, check query length or use @solana/web3.js PublicKey validation
  const isAddress = query.length >= 32; 

  return (
    <div className="min-h-[100vh] flex flex-col items-center justify-center p-6 bg-zinc-50 dark:bg-black">
      <div className="w-full max-w-2xl text-center">
        
        {/* Visual Hook */}
        <div className="mb-8 inline-flex items-center justify-center w-20 h-20 rounded-full bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 shadow-2xl">
          <Fingerprint size={40} className="text-emerald-500 animate-pulse" />
        </div>

        <h1 className="text-4xl md:text-6xl font-black tracking-tighter mb-4 dark:text-white">
          VERIFY REPUTATION
        </h1>
        <p className="text-zinc-500 dark:text-zinc-400 mb-12 font-medium">
          Enter a Solana wallet address, PDA, or Stature ID to view the immutable record.
        </p>

        {/* The "Smack Down" Search Box */}
        <div className="relative group">
          <div className="absolute -inset-1 bg-gradient-to-r from-emerald-500 to-blue-600 rounded-[2rem] blur opacity-25 group-focus-within:opacity-50 transition duration-1000"></div>
          
          <div className="relative flex items-center bg-white dark:bg-zinc-900 border-2 border-zinc-200 dark:border-zinc-800 rounded-3xl p-2 shadow-xl">
            <div className="pl-4 text-zinc-400">
              <Search size={24} />
            </div>
            <input 
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Address / PDA / Name..."
              className="w-full bg-transparent border-none focus:ring-0 px-4 py-4 text-lg font-mono placeholder:text-zinc-400 dark:text-white"
            />
            <button className="bg-black dark:bg-white dark:text-black text-white px-6 py-3 rounded-2xl font-bold hover:scale-95 transition-transform flex items-center gap-2">
              LOOKUP
            </button>
          </div>
        </div>

        {/* Dynamic Preview Results (Shows up as you type) */}
        {query.length > 3 && (
          <div className="mt-8 space-y-3 animate-in fade-in slide-in-from-top-4 duration-300">
            <p className="text-[10px] font-bold text-zinc-400 uppercase tracking-[0.2em]">Suggested Matches</p>
            
            {/* Example: User Match */}
            <SearchResultItem 
              type="USER" 
              name="S. Engineer" 
              address="7xWp...3u9Y" 
              stature={1250}
              icon={<User size={18} />}
            />

            {/* Example: Company Match */}
            <SearchResultItem 
              type="COMPANY" 
              name="DevShop Delta" 
              address="Delta...92k1" 
              stature={4500}
              icon={<Building2 size={18} />}
            />
          </div>
        )}

        {/* Trust Indicators */}
        <div className="mt-20 flex flex-wrap justify-center gap-8 opacity-40 grayscale hover:grayscale-0 transition-all">
          <div className="flex items-center gap-2 font-black tracking-tighter italic text-xl">
            <Shield className="text-emerald-500" /> SOLANA
          </div>
          <div className="flex items-center gap-2 font-black tracking-tighter italic text-xl">
             AMPLIFY GEN 2
          </div>
          <div className="flex items-center gap-2 font-black tracking-tighter italic text-xl">
            STATURE v1.0
          </div>
        </div>
      </div>
    </div>
  );
}

// --- Search Result Item ---

function SearchResultItem({ type, name, address, stature, icon }: any) {
  return (
    <div className="group flex items-center justify-between p-4 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-2xl cursor-pointer hover:border-emerald-500 transition-all shadow-sm">
      <div className="flex items-center gap-4">
        <div className="p-2 rounded-xl bg-zinc-100 dark:bg-zinc-800 text-zinc-500 group-hover:bg-emerald-500 group-hover:text-white transition-colors">
          {icon}
        </div>
        <div className="text-left">
          <div className="flex items-center gap-2">
            <span className="text-xs font-bold text-emerald-500">{type}</span>
            <h4 className="font-bold text-sm dark:text-white">{name}</h4>
          </div>
          <p className="text-[10px] font-mono text-zinc-400">{address}</p>
        </div>
      </div>
      <div className="flex items-center gap-4">
        <div className="text-right">
          <p className="text-[10px] font-bold text-zinc-400 uppercase">Stature</p>
          <p className="font-black dark:text-white">{stature}</p>
        </div>
        <ArrowRight size={18} className="text-zinc-300 group-hover:text-emerald-500 transition-colors" />
      </div>
    </div>
  );
}