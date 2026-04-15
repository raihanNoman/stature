"use client";

import React from "react";
import { Check } from "lucide-react";

interface Step {
  title: string;
  desc: string;
}

export default function StepsFlow({ steps }: { steps: Step[] }) {
  return (
    <div className="mt-16 w-full">
      {/* Mobile: Vertical Timeline | Desktop: Horizontal Row */}
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-8 md:gap-4">
        {steps.map((step, i) => (
          <React.Fragment key={i}>
            <div className="group relative flex flex-row md:flex-col items-center md:text-center gap-4 md:flex-1">
              {/* The Number/Icon Circle */}
              <div className="relative z-10 flex h-12 w-12 shrink-0 items-center justify-center rounded-full bg-black text-white dark:bg-zinc-100 dark:text-black font-black text-sm transition-transform group-hover:scale-110">
                {i + 1}
                {/* Glow effect on hover */}
                <div className="absolute inset-0 rounded-full bg-emerald-500 blur-md opacity-0 group-hover:opacity-40 transition-opacity" />
              </div>

              {/* Text Content */}
              <div className="flex flex-col">
                <h4 className="text-xs font-black uppercase tracking-widest text-zinc-900 dark:text-white">
                  {step.title}
                </h4>
                
                {/* Modern Hover Reveal Description */}
                <p className="mt-2 text-sm text-zinc-500 dark:text-zinc-400 opacity-0 md:absolute md:top-16 md:left-1/2 md:-translate-x-1/2 md:w-48 group-hover:opacity-100 transition-all duration-300 pointer-events-none bg-zinc-50 dark:bg-zinc-900 p-2 rounded-lg border border-zinc-200 dark:border-zinc-800 shadow-xl z-20 md:block hidden">
                  {step.desc}
                </p>
                {/* Always visible on mobile for UX */}
                <p className="md:hidden text-xs text-zinc-500 mt-1">
                  {step.desc}
                </p>
              </div>
            </div>

            {/* Connecting Line (Hidden on last item) */}
            {i !== steps.length - 1 && (
              <div className="hidden md:block h-[2px] flex-1 bg-zinc-200 dark:bg-zinc-800 mx-4" />
            )}
            {i !== steps.length - 1 && (
              <div className="md:hidden w-[2px] h-8 bg-zinc-200 dark:bg-zinc-800 ml-6" />
            )}
          </React.Fragment>
        ))}
      </div>
    </div>
  );
}