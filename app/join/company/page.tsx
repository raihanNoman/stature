import React from "react";
import {
  CheckCircle2,
  PlayCircle,
  ShieldCheck,
  Zap,
  BarChart3,
} from "lucide-react";
import RegisterCompanyForm from "@/app/components/join/program";
import StepsFlow from "@/app/components/StepsFlow";

// The Company Journey Logic
const COMPANY_STEPS = [
  {
    title: "Registration",
    desc: "Initialize your Company PDA on Solana to establish your cryptographic presence.",
  },
  {
    title: "Probation",
    desc: "Your account enters a review state where our team verifies your business legitimacy.",
  },
  {
    title: "Verification",
    desc: "Once cleared, you receive 'Verified' status and your specific Stature Weight multiplier.",
  },
  {
    title: "Issue Stature",
    desc: "You now have the power to sign and hash work records for your users on the ledger.",
  },
];

export default function JoinAsCompany() {
  return (
    <div className="min-h-screen bg-zinc-50 text-zinc-900 dark:bg-black dark:text-zinc-100 font-sans">
      <main className="max-w-6xl mx-auto px-6 py-20">
        {/* Header Section */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          <div>
            <span className="text-emerald-500 font-bold uppercase tracking-widest text-xs">
              Partner Program
            </span>
            <h1 className="mt-4 text-5xl md:text-7xl font-black tracking-tighter leading-[0.9]">
              BECOME A <br />
              <span className="text-zinc-400 dark:text-zinc-800 italic uppercase">
                Verified Entity.
              </span>
            </h1>
            <p className="mt-8 text-xl text-zinc-600 dark:text-zinc-400 leading-relaxed max-w-lg">
              Only verified companies have the power to assign Stature.
              Transform your business transactions into immutable proof of
              quality.
            </p>
            {/* The New Steps Section - Replaces the BenefitItem list */}
            <div className="mt-32">
              <div className="flex flex-col items-center mb-12">
                <h2 className="text-sm font-black uppercase tracking-[0.3em] text-zinc-400">
                  Onboarding Protocol
                </h2>
                <div className="mt-2 h-1 w-12 bg-emerald-500" />
              </div>
              <StepsFlow steps={COMPANY_STEPS} />
            </div>
          </div>

          {/* Video Placeholder Section */}
          <div className="group relative aspect-video rounded-[2.5rem] bg-zinc-200 dark:bg-zinc-900 border border-zinc-300 dark:border-zinc-800 overflow-hidden flex items-center justify-center shadow-2xl">
            <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1639762681485-074b7f938ba0?q=80&w=2832&auto=format&fit=crop')] bg-cover opacity-20 grayscale group-hover:scale-105 transition-transform duration-700" />
            <PlayCircle
              size={64}
              className="relative z-10 text-emerald-500 cursor-pointer hover:scale-110 transition-transform"
            />
            <p className="absolute bottom-6 left-6 font-bold text-xs uppercase tracking-widest">
              How Stature Works for Business
            </p>
          </div>
        </div>

        {/* The Registration Form (Client Component) */}
        <section id="register" className="mt-32 max-w-3xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold uppercase tracking-tight">
              Register Your Identity
            </h2>
            <p className="text-zinc-500 mt-2 italic">
              Requires a one-time Solana transaction to initialize your Company
              PDA.
            </p>
          </div>
          <RegisterCompanyForm />
        </section>

        {/* Deep Dive Section */}
        <section className="mt-40 grid grid-cols-1 md:grid-cols-3 gap-12 border-t border-zinc-200 dark:border-zinc-800 pt-20">
          <InfoCard
            icon={<ShieldCheck className="text-blue-500" />}
            title="Protocol Verification"
            desc="Every company is reviewed by our core team. This ensures the Stature issued within the network remains untainted and high-value."
          />
          <InfoCard
            icon={<BarChart3 className="text-emerald-500" />}
            title="Network Intelligence"
            desc="Query any wallet or PDA before you sign a contract. Know exactly who you are working with based on their historical output."
          />
          <InfoCard
            icon={<Zap className="text-yellow-500" />}
            title="Programmable Trust"
            desc="Coming soon: Automated money movement tied to reputation thresholds. Smart contracts that pay based on Stature score."
          />
        </section>
      </main>
    </div>
  );
}

function BenefitItem({ text }: { text: string }) {
  return (
    <div className="flex items-center gap-3 font-bold text-sm tracking-tight uppercase">
      <CheckCircle2 size={18} className="text-emerald-500" />
      {text}
    </div>
  );
}

function InfoCard({ icon, title, desc }: any) {
  return (
    <div>
      <div className="mb-4">{icon}</div>
      <h3 className="font-black uppercase tracking-tight mb-2">{title}</h3>
      <p className="text-sm text-zinc-500 leading-relaxed italic">{desc}</p>
    </div>
  );
}
