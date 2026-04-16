"use client";

import React, { useState } from 'react';
import { ShieldAlert, Zap, Cpu, Loader2 } from 'lucide-react';

export default function InitAdminPage() {
  const [loading, setLoading] = useState(false);

  const handleInitialize = async () => {
    setLoading(true);
    // Logic: program.methods.initializeConfig().rpc()
    setTimeout(() => setLoading(false), 2000);
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-6 bg-zinc-50 dark:bg-black">
      <div className="max-w-md w-full bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-[2.5rem] p-10 shadow-2xl text-center">
        <div className="w-16 h-16 bg-red-500/10 text-red-500 rounded-2xl flex items-center justify-center mx-auto mb-6">
          <ShieldAlert size={32} />
        </div>
        
        <h1 className="text-2xl font-black uppercase tracking-tighter mb-2">Protocol Genesis</h1>
        <p className="text-sm text-zinc-500 mb-8 italic">
          Initialize the Stature Config account. This sets the global admin authority and protocol fee structures.
        </p>

        <div className="space-y-4 mb-8">
          <div className="flex items-center gap-3 p-4 bg-zinc-50 dark:bg-zinc-800/50 rounded-xl border border-zinc-200 dark:border-zinc-700 text-left">
            <Cpu size={18} className="text-zinc-400" />
            <div>
              <p className="text-[10px] font-bold text-zinc-400 uppercase">Target Program</p>
              <p className="text-xs font-mono">Stat...82uY</p>
            </div>
          </div>
        </div>

        <button 
          onClick={handleInitialize}
          disabled={loading}
          className="w-full h-14 bg-red-600 hover:bg-red-700 text-white rounded-xl font-black uppercase tracking-widest text-xs flex items-center justify-center gap-2 transition-all disabled:opacity-50"
        >
          {loading ? <Loader2 className="animate-spin" /> : <><Zap size={16} fill="currentColor" /> Initialize Config</>}
        </button>
      </div>
    </div>
  );
}