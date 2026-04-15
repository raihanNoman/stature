"use client";

import React, { useState } from "react";
import { Loader2, UserPlus, Shield } from "lucide-react";

export default function RegisterUserForm() {
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState("");

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    // 1. Check wallet connection
    // 2. Call Anchor initialize_user(name)
    // 3. Create UserProfile in Amplify
    setTimeout(() => setLoading(false), 2000);
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