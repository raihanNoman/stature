"use client";

import React, { useState } from "react";
import { Upload, ChevronRight, Loader2 } from "lucide-react";

export default function RegisterCompanyForm() {
  const [loading, setLoading] = useState(false);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      // 1. SOLANA: Call Anchor instruction 'initialize_company'
      // const tx = await program.methods.initializeCompany(name)...

      // 2. AMPLIFY: Store rich metadata using PDA as ID
      // await client.models.CompanyProfile.create({ id: companyPda, name, description })

      console.log("Registering...", { name, description });
      // Simulate delay
      await new Promise(r => setTimeout(r, 2000));
      alert("Registration request submitted for verification!");
      
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <form 
      onSubmit={handleRegister}
      className="bg-white dark:bg-zinc-900/50 border border-zinc-200 dark:border-zinc-800 p-8 md:p-12 rounded-[2.5rem] shadow-xl"
    >
      <div className="space-y-6">
        <div>
          <label className="block text-[10px] font-bold uppercase tracking-widest text-zinc-400 mb-2">Company Name</label>
          <input 
            required
            type="text" 
            placeholder="e.g. Delta Software"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full bg-zinc-50 dark:bg-black border border-zinc-200 dark:border-zinc-800 rounded-xl px-5 py-4 font-bold placeholder:opacity-30 focus:ring-2 focus:ring-emerald-500 outline-none transition-all"
          />
        </div>

        <div>
          <label className="block text-[10px] font-bold uppercase tracking-widest text-zinc-400 mb-2">Business Description</label>
          <textarea 
            required
            rows={4}
            placeholder="What does your company do?"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="w-full bg-zinc-50 dark:bg-black border border-zinc-200 dark:border-zinc-800 rounded-xl px-5 py-4 font-medium placeholder:opacity-30 focus:ring-2 focus:ring-emerald-500 outline-none transition-all resize-none"
          />
        </div>

        <div className="p-6 border-2 border-dashed border-zinc-200 dark:border-zinc-800 rounded-2xl flex flex-col items-center justify-center text-center cursor-pointer hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors">
          <Upload size={24} className="text-zinc-400 mb-2" />
          <p className="text-xs font-bold uppercase">Upload Company Logo</p>
          <p className="text-[10px] text-zinc-500 mt-1 italic">SVG or PNG (Max 2MB)</p>
        </div>

        <button 
          disabled={loading}
          type="submit"
          className="w-full h-16 bg-black dark:bg-white text-white dark:text-black rounded-2xl font-black uppercase tracking-widest text-sm flex items-center justify-center gap-2 hover:scale-[0.98] transition-transform disabled:opacity-50"
        >
          {loading ? <Loader2 className="animate-spin" /> : <>Initialize on Solana <ChevronRight size={18} /></>}
        </button>
      </div>
    </form>
  );
}