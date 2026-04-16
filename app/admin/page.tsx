"use client";

import React, { useState } from "react";
import {
  ShieldCheck,
  Mail,
  X,
  ExternalLink,
  Globe,
  Lock,
  UserCog,
} from "lucide-react";
import { useWallet } from "../lib/wallet/context";

export default function AdminPage() {
  const { wallet, signer, status } = useWallet();

  const adminWallet= wallet?.account.address
  return (
    <div className="min-h-screen bg-zinc-50 dark:bg-[#0a0a0a] text-zinc-900 dark:text-zinc-100 p-6 md:p-12">
      <div className="max-w-4xl mx-auto">
        {/* Header Section */}
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-16">
          <div>
            <h1 className="text-4xl font-black tracking-tighter uppercase italic">
              Protocol Admin
            </h1>
            <p className="text-zinc-500 text-sm mt-1 font-medium">
              Stature Network Global Configuration Dashboard
            </p>
          </div>
          <div className="flex items-center gap-2 px-4 py-2 bg-emerald-500/10 border border-emerald-500/20 rounded-full">
            <div className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
            <span className="text-[10px] font-black uppercase text-emerald-500 tracking-widest">
              Mainnet-Beta Active
            </span>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {/* Status Card */}
          <div className="bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-[2.5rem] p-8 shadow-sm">
            <div className="flex items-center gap-3 mb-6">
              <UserCog size={20} className="text-zinc-400" />
              <h3 className="text-xs font-bold uppercase tracking-[0.2em] text-zinc-400">
                Current Authority
              </h3>
            </div>

            {adminWallet ? (
              <div className="space-y-4">
                <div className="p-4 bg-zinc-50 dark:bg-black rounded-2xl border border-zinc-200 dark:border-zinc-800">
                  <p className="text-[10px] font-bold text-zinc-400 uppercase mb-1">
                    Admin Wallet ID
                  </p>
                  <p className="text-sm font-mono font-bold break-all">
                    {adminWallet}
                  </p>
                </div>
                <div className="flex items-center gap-2 text-emerald-500">
                  <ShieldCheck size={16} />
                  <span className="text-[10px] font-black uppercase tracking-widest">
                    Ownership Verified on Ledger
                  </span>
                </div>
              </div>
            ) : (
              <div className="p-6 text-center border-2 border-dashed border-zinc-200 dark:border-zinc-800 rounded-2xl">
                <Lock size={24} className="mx-auto text-zinc-300 mb-2" />
                <p className="text-xs font-bold text-zinc-400 uppercase">
                  No Admin Found
                </p>
                <p className="text-[10px] text-zinc-500 mt-1 italic italic">
                  Initialize Protocol via /admin/init
                </p>
              </div>
            )}
          </div>

          {/* Contact & MVP Support Card */}
          <div className="bg-black text-white rounded-[2.5rem] p-8 shadow-2xl relative overflow-hidden">
            {/* Decorative Background Element */}
            <div className="absolute top-0 right-0 p-4 opacity-10">
              <Globe size={120} />
            </div>

            <h3 className="text-xs font-bold uppercase tracking-[0.2em] text-zinc-500 mb-6 relative z-10">
              MVP Support & Feedback
            </h3>

            <div className="space-y-4 relative z-10">
              <ContactLink
                href="https://twitter.com/stature_net"
                icon={<X size={18} />}
                label="Twitter Handler"
                value="@stature_network"
              />
              <ContactLink
                href="mailto:admin@stature.network"
                icon={<Mail size={18} />}
                label="Direct Email"
                value="admin@stature.network"
              />
            </div>

            <p className="mt-8 text-[10px] leading-relaxed text-zinc-500 italic">
              Found a bug or need urgent company verification? Reach out via
              Twitter DM for the fastest response.
            </p>
          </div>
        </div>

        {/* Quick Actions Footer */}
        <div className="mt-12 flex gap-4 overflow-x-auto pb-4">
          <QuickAction href="/admin/verify/company" label="Verify Companies" />
          <QuickAction href="/admin/init" label="Protocol Settings" />
          <QuickAction
            href="https://solscan.io"
            label="View Program on Solscan"
            external
          />
        </div>
      </div>
    </div>
  );
}

// --- Helper Components ---

function ContactLink({ href, icon, label, value }: any) {
  return (
    <a
      href={href}
      target="_blank"
      className="flex items-center gap-4 p-4 rounded-2xl bg-zinc-900 border border-zinc-800 hover:border-emerald-500 transition-all group"
    >
      <div className="text-zinc-500 group-hover:text-emerald-500 transition-colors">
        {icon}
      </div>
      <div>
        <p className="text-[10px] font-bold text-zinc-500 uppercase leading-none mb-1">
          {label}
        </p>
        <p className="text-sm font-bold tracking-tight">{value}</p>
      </div>
    </a>
  );
}

function QuickAction({ href, label, external = false }: any) {
  return (
    <a
      href={href}
      target={external ? "_blank" : "_self"}
      className="shrink-0 px-6 py-3 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl text-[10px] font-bold uppercase tracking-widest hover:border-black dark:hover:border-white transition-all flex items-center gap-2"
    >
      {label} {external && <ExternalLink size={10} />}
    </a>
  );
}
