import React from "react";

import { PlayCircle, Award, Globe, Fingerprint } from "lucide-react";
import StepsFlow from "@/app/components/StepsFlow";
import RegisterUserForm from "@/app/components/join/user";

const USER_STEPS = [
  {
    title: "Secure Identity",
    desc: "Initialize your User Account PDA on Solana to start your ledger.",
  },
  {
    title: "Proof of Work",
    desc: "Collaborate with verified companies. Your impact is signed and hashed.",
  },
  {
    title: "Earn Stature",
    desc: "Watch your reputation score grow as you accumulate verified records.",
  },
  {
    title: "Global Passport",
    desc: "Use your Stature score to bypass credit checks and access premium pools.",
  },
];

export default function JoinAsUser() {
  return (
    <div className="min-h-screen bg-zinc-50 dark:bg-black text-zinc-900 dark:text-zinc-100">
      <main className="max-w-6xl mx-auto px-6 py-20">
        {/* Steps Section */}
        <div className="mt-12 mb-24">
          <h2 className="text-center text-sm font-black uppercase tracking-[0.3em] text-zinc-400">
            The User Journey
          </h2>
          <StepsFlow steps={USER_STEPS} />
        </div>

        {/* Hero Section */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          <div>
            <span className="text-blue-500 font-bold uppercase tracking-widest text-[10px]">
              Individual Identity
            </span>
            <h1 className="mt-4 text-5xl md:text-7xl font-black tracking-tighter leading-[0.9]">
              YOUR CHARACTER <br />
              <span className="text-zinc-400 dark:text-zinc-800 italic uppercase">
                On-Chain.
              </span>
            </h1>
            <p className="mt-8 text-xl text-zinc-600 dark:text-zinc-400 leading-relaxed max-w-lg">
              Stop relying on resumes that anyone can fake. Build a portable,
              verified reputation that follows you across the decentralized web.
            </p>
          </div>

          <div className="group relative aspect-video rounded-[2.5rem] bg-zinc-200 dark:bg-zinc-900 border border-zinc-300 dark:border-zinc-800 overflow-hidden flex items-center justify-center shadow-2xl">
            <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1614064641935-44cfe65dceec?q=80&w=2940&auto=format&fit=crop')] bg-cover opacity-10 grayscale group-hover:scale-105 transition-all duration-700" />
            <PlayCircle
              size={64}
              className="relative z-10 text-blue-500 cursor-pointer hover:scale-110 transition-transform"
            />
          </div>
        </div>

        {/* Registration Form */}
        <section id="register" className="mt-40 max-w-3xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold uppercase tracking-tight">
              Claim Your User ID
            </h2>
            <p className="text-zinc-500 mt-2 font-mono text-xs italic">
              solana.program.initialize_user()
            </p>
          </div>
          <RegisterUserForm />
        </section>

        {/* Benefits Grid */}
        <section className="mt-40 grid grid-cols-1 md:grid-cols-3 gap-12 border-t border-zinc-200 dark:border-zinc-800 pt-20">
          <Feature
            icon={<Award />}
            title="Resume Killer"
            desc="Why tell them you're good? Let the companies you've worked with prove it for you on the ledger."
          />
          <Feature
            icon={<Globe />}
            title="Global Trust"
            desc="Whether you are in Dhaka, New York, or Tokyo—your Stature is recognized everywhere."
          />
          <Feature
            icon={<Fingerprint />}
            title="Soulbound"
            desc="Your score cannot be transferred, sold, or stolen. It is uniquely yours."
          />
        </section>
      </main>
    </div>
  );
}

function Feature({ icon, title, desc }: any) {
  return (
    <div className="text-left">
      <div className="mb-4 text-blue-500">{icon}</div>
      <h3 className="font-black uppercase tracking-tight text-sm mb-2">
        {title}
      </h3>
      <p className="text-sm text-zinc-500 dark:text-zinc-400 leading-relaxed italic">
        {desc}
      </p>
    </div>
  );
}
