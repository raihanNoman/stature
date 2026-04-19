"use client";
import Link from "next/link";
import { Zap } from "lucide-react";
import SearchUtility from "./components/serach";
import PostUpdateStature_Btn from "./lib/aws-rest-api-test";
import HowItWorks from "./components/how-it-works";

export default function StatureHome() {
  return (
    <div className="flex min-h-screen flex-col bg-zinc-50 font-sans text-zinc-900 transition-colors duration-300 dark:bg-[#0a0a0a] dark:text-zinc-100">
      {/* Navigation */}
      <nav className="fixed top-0 z-50 w-full flex items-center justify-between px-6 py-6 md:px-12 bg-zinc-50/80 dark:bg-[#0a0a0a]/80 backdrop-blur-md border-b border-zinc-200/50 dark:border-zinc-800/50">
        <Link href="/">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-black text-white dark:bg-zinc-100 dark:text-black">
              <Zap size={20} fill="currentColor" />
            </div>
            <span className="text-xl font-black tracking-tighter uppercase">
              Stature
            </span>
          </div>
        </Link>

        <div className="hidden gap-8 text-[10px] font-bold uppercase tracking-widest text-zinc-500 md:flex">
          <a
            href="#logic"
            className="hover:text-black dark:hover:text-white transition-colors"
          >
            The Logic
          </a>
          <a
            href="#pillars"
            className="hover:text-black dark:hover:text-white transition-colors"
          >
            Pillars
          </a>
        </div>
        <button className="rounded-full bg-black px-5 py-2 text-xs font-bold text-white transition-all hover:scale-105 dark:bg-white dark:text-black">
          Connect Wallet
        </button>
      </nav>

      {/* Hero & Search Utility */}
      <section className="relative flex h-[85vh] flex-col items-center justify-center px-6 pt-20 text-center">
        <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-emerald-500/10 blur-[120px] rounded-full" />

        <h1 className="max-w-5xl text-5xl font-black leading-[0.9] tracking-tighter sm:text-7xl md:text-8xl">
          REPUTATION IS <br />
          <span className="text-zinc-400 dark:text-zinc-800 uppercase italic">
            Not an Opinion.
          </span>
        </h1>

        <SearchUtility />

        {/* --- Action Buttons --- */}
        <div className="mt-8 flex flex-col sm:flex-row items-center gap-4">
          <Link href="/join/user" className="group relative">
            <div className="absolute -inset-0.5 bg-gradient-to-r from-emerald-500 to-blue-500 rounded-xl blur opacity-20 group-hover:opacity-50 transition duration-300"></div>
            <button className="relative flex h-14 items-center justify-center rounded-xl bg-black px-8 text-sm font-black uppercase tracking-widest text-white transition-transform active:scale-95 dark:bg-zinc-100 dark:text-black">
              Join as User
            </button>
          </Link>

          <Link href="/join/company">
            <button className="flex h-14 items-center justify-center rounded-xl border-2 border-zinc-200 px-8 text-sm font-black uppercase tracking-widest transition-all hover:bg-zinc-100 active:scale-95 dark:border-zinc-800 dark:hover:bg-zinc-900">
              Register Company
            </button>
          </Link>
        </div>

        <p className="mt-8 text-sm font-mono text-zinc-500 uppercase tracking-widest">
          Standardizing trust on the Solana Ledger
        </p>

        <div className="absolute bottom-10 animate-bounce">
          <div className="w-px h-12 bg-zinc-300 dark:bg-zinc-700" />
        </div>
      </section>

      {/* Content Sections */}
      <main className="flex flex-col items-center px-6 py-20 md:px-12">
        {/* <PostUpdateStature_Btn /> */}
        <section id="logic">
          <HowItWorks />
        </section>

        <section className="w-full max-w-6xl">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div className="rounded-[2.5rem] bg-zinc-100 p-10 dark:bg-zinc-900/40 border border-zinc-200 dark:border-zinc-800">
              <span className="text-red-500 font-bold uppercase tracking-widest text-[10px]">
                The Past
              </span>
              <h2 className="mt-4 text-3xl font-black tracking-tight uppercase">
                The Review Economy
              </h2>
              <ul className="mt-8 space-y-4 text-sm font-medium text-zinc-500 italic">
                <li>✕ Anonymous "Karens" destroying businesses</li>
                <li>✕ Bot-generated 5-star reviews</li>
                <li>✕ Zero proof of transaction</li>
              </ul>
            </div>

            <div className="rounded-[2.5rem] bg-black p-10 text-white shadow-2xl shadow-emerald-500/10">
              <span className="text-emerald-400 font-bold uppercase tracking-widest text-[10px]">
                The Future
              </span>
              <h2 className="mt-4 text-3xl font-black tracking-tight uppercase">
                The Stature Network
              </h2>
              <ul className="mt-8 space-y-4 text-sm font-medium text-zinc-300">
                <li>✓ Soulbound Work Credentials</li>
                <li>✓ Witness-Verified Contracts (2:282)</li>
                <li>✓ Immutable Solana Ledger</li>
              </ul>
            </div>
          </div>
        </section>

        <section id="pillars" className="mt-40 w-full max-w-4xl">
          <h2 className="text-4xl font-black tracking-tighter uppercase italic text-center mb-20">
            The Pillars of Stature
          </h2>
          <div className="space-y-12">
            <Pillar
              title="Soulbound Identity"
              desc="Your score is tied to your wallet. It cannot be bought, only built through verified output."
            />
            <Pillar
              title="Cross-App Utility"
              desc="One score for renting, hiring, and insurance pools like Takaful Club."
            />
            <Pillar
              title="On-Chain Integrity"
              desc="Every reputation point is backed by a signed transaction hash on Solana."
            />
          </div>
        </section>
      </main>

      <footer className="mt-40 border-t border-zinc-200 py-12 px-6 dark:border-zinc-800 md:px-12 flex flex-col md:flex-row justify-between items-center gap-6">
        <span className="text-xs font-bold uppercase tracking-widest opacity-50">
          Stature.Network © 2026
        </span>
        <div className="flex gap-8">
          <Link
            href="https://contract282.com"
            className="text-[10px] font-bold hover:underline tracking-widest uppercase"
          >
            CONTRACT 282
          </Link>
          <Link
            href="https://takafulclub.com"
            className="text-[10px] font-bold hover:underline tracking-widest uppercase"
          >
            TAKAFUL CLUB
          </Link>
        </div>
      </footer>
    </div>
  );
}

function Pillar({ title, desc }: { title: string; desc: string }) {
  return (
    <div className="group border-b border-zinc-200 dark:border-zinc-800 pb-8 transition-colors hover:border-black dark:hover:border-white">
      <h3 className="text-xl font-bold uppercase tracking-tight">{title}</h3>
      <p className="mt-2 text-zinc-500 dark:text-zinc-400 leading-relaxed">
        {desc}
      </p>
    </div>
  );
}
