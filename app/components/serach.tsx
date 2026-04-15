"use client";

import React, { useState } from "react";
import { Search, ArrowRight, User, Building2 } from "lucide-react";
import { useRouter } from "next/navigation";

export default function SearchUtility() {
  const [query, setQuery] = useState("");
  const router = useRouter();

  // In production, this would hit your Amplify API or Solana RPC
  const handleLookup = () => {
    if (query.length > 20) {
      // Logic to determine if it's a User or Company PDA would go here
      router.push(`/lookup/${query}`);
    }
  };

  return (
    <div className="mt-12 w-full max-w-2xl relative group">
      {/* Glow Effect */}
      <div className="absolute -inset-1 bg-gradient-to-r from-emerald-500 to-blue-600 rounded-[2rem] blur opacity-20 group-focus-within:opacity-40 transition duration-1000"></div>
      
      <div className="relative flex items-center bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-[1.5rem] p-1.5 shadow-2xl">
        <div className="pl-4 text-zinc-400">
          <Search size={22} />
        </div>
        <input 
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleLookup()}
          placeholder="Enter Wallet Address or PDA..."
          className="w-full bg-transparent border-none focus:ring-0 px-4 py-3 text-base md:text-lg font-mono placeholder:text-zinc-400 dark:text-white"
        />
        <button 
          onClick={handleLookup}
          className="hidden md:flex bg-black dark:bg-white dark:text-black text-white px-6 py-3 rounded-2xl font-bold text-sm items-center gap-2"
        >
          LOOKUP
        </button>
      </div>

      {/* Instant Results Dropdown (Dummy Data) */}
      {query.length > 5 && (
        <div className="absolute z-10 top-full mt-4 w-full bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-3xl p-2 shadow-2xl animate-in fade-in slide-in-from-top-2">
           <SearchResultItem type="USER" name="S. Engineer" stature={1250} icon={<User size={16}/>} />
           <SearchResultItem type="COMPANY" name="DevShop Delta" stature={4500} icon={<Building2 size={16}/>} />
        </div>
      )}
    </div>
  );
}

function SearchResultItem({ type, name, stature, icon }: any) {
  return (
    <div className="flex items-center justify-between p-3 hover:bg-zinc-50 dark:hover:bg-zinc-800/50 rounded-2xl cursor-pointer group transition-colors text-zinc-900 dark:text-zinc-100">
      <div className="flex items-center gap-3">
        <div className="p-2 bg-zinc-100 dark:bg-zinc-800 rounded-lg group-hover:bg-emerald-500 group-hover:text-white transition-colors">
          {icon}
        </div>
        <div className="text-left">
          <p className="text-[10px] font-bold text-emerald-500 uppercase tracking-tighter leading-none mb-1">{type}</p>
          <p className="font-bold text-sm">{name}</p>
        </div>
      </div>
      <div className="text-right flex items-center gap-4">
        <div>
           <p className="text-[8px] font-bold text-zinc-400 uppercase">Stature</p>
           <p className="font-black text-sm">{stature}</p>
        </div>
        <ArrowRight size={14} className="opacity-0 group-hover:opacity-100 transition-opacity" />
      </div>
    </div>
  );
}