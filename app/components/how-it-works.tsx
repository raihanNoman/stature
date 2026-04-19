import React from "react";

/**
 * STURE.NETWORK "How it Works" Component
 * Logic:
 * 1. Logarithmic Value Weighting: P = log10(Value) * DomainWeight
 * 2. Exponential Level Curve: TotalPoints = 100 * (Level^2)
 */

export default function HowItWorks() {
  const domainWeights = [
    { name: "Micro-Finance", weight: 1.0, color: "bg-blue-500" },
    { name: "Auto Insurance", weight: 5.0, color: "bg-purple-500" },
    { name: "Legal / Agreements", weight: 8.0, color: "bg-emerald-500" },
  ];

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 dark:bg-slate-950 dark:text-slate-100 transition-colors duration-300 font-sans">
      {/* Hero Section */}
      <div className="px-6 py-16 md:py-24 max-w-6xl mx-auto text-center">
        <h1 className="text-4xl md:text-6xl font-extrabold tracking-tight mb-6 bg-gradient-to-r from-blue-600 to-emerald-500 bg-clip-text text-transparent">
          The Science of Stature
        </h1>
        <p className="text-lg md:text-xl text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
          A reputation protocol that scales with integrity, not just capital.
          Fair for the hustler, secure against bots, and respected by the
          whales.
        </p>
      </div>

      <hr className="border-slate-200 dark:border-slate-800 max-w-4xl mx-auto" />

      {/* The Core Logic Grid */}
      <div className="px-6 py-16 max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
        <div>
          <h2 className="text-3xl font-bold mb-4">
            1. The Anti-Whale Equation
          </h2>
          <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed">
            We use{" "}
            <span className="font-mono text-blue-500 font-bold">
              Logarithmic Scaling
            </span>{" "}
            for transaction values. This means a $1,000,000 deposit doesn't give
            you a million times more reputation than a $10 deposit. It proves
            you have the funds, but forces you to show consistency to earn real
            trust.
          </p>
          <div className="p-4 bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm">
            <code className="text-sm md:text-base font-mono">
              Points = log10(Value) × DomainWeight
            </code>
          </div>
        </div>
        <div className="bg-slate-200 dark:bg-slate-800 h-64 rounded-2xl flex items-center justify-center p-8">
          {/* Visual representation of a Log Curve would go here */}
          <div className="w-full h-full border-b-2 border-l-2 border-slate-400 relative flex items-end">
            <div className="absolute bottom-0 left-0 w-full h-full overflow-hidden">
              <svg
                viewBox="0 0 100 100"
                className="w-full h-full stroke-blue-500 fill-none stroke-2"
              >
                <path d="M 0 100 Q 10 20 100 0" />
              </svg>
            </div>
            <span className="absolute -bottom-6 left-0 text-xs">Value ($)</span>
            <span className="absolute -left-8 top-0 text-xs rotate-90 origin-bottom-left">
              Stature
            </span>
          </div>
        </div>
      </div>

      {/* Domain Weights */}
      <div className="px-6 py-16 bg-slate-100 dark:bg-slate-900/50">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-3xl font-bold mb-8 text-center">
            2. Domain Weights
          </h2>
          <p className="text-center text-slate-600 dark:text-slate-400 mb-10">
            Not all work is created equal. High-integrity industries carry
            higher reputation multipliers.
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            {domainWeights.map((item) => (
              <div
                key={item.name}
                className="p-6 bg-white dark:bg-slate-800 rounded-2xl shadow-sm border border-slate-200 dark:border-slate-700"
              >
                <div className={`h-2 w-12 rounded-full ${item.color} mb-4`} />
                <h3 className="font-bold text-lg">{item.name}</h3>
                <p className="text-2xl font-mono mt-2">
                  x{item.weight.toFixed(1)}
                </p>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Progression Levels */}
      <div className="px-6 py-16 max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold mb-4">3. The Climb to Level 100</h2>
          <p className="text-slate-600 dark:text-slate-400">
            Trust is earned through the "Test of Time."
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 text-center">
          <div className="space-y-4">
            <div className="text-4xl font-black text-slate-300 dark:text-slate-700">
              Lvl 1-10
            </div>
            <h4 className="font-bold">The Sandbox</h4>
            <p className="text-sm text-slate-500">
              Proving you are human. Micro-transactions and social verification.
            </p>
          </div>
          <div className="space-y-4 p-8 bg-blue-600 rounded-3xl text-white transform md:-translate-y-4 shadow-xl">
            <div className="text-4xl font-black opacity-50">Lvl 50</div>
            <h4 className="font-bold">The Professional</h4>
            <p className="text-sm opacity-90">
              Unlock lower collateral, high-limit contracts, and governance
              rights.
            </p>
          </div>
          <div className="space-y-4">
            <div className="text-4xl font-black text-slate-300 dark:text-slate-700">
              Lvl 100
            </div>
            <h4 className="font-bold">The Institution</h4>
            <p className="text-sm text-slate-500">
              Peak network trust. Reserved for consistent, high-value ecosystem
              pillars.
            </p>
          </div>
        </div>
      </div>

      <MathSection />
      {/* Footer / Call to Action */}
      <div className="px-6 py-16 text-center bg-white dark:bg-slate-950 border-t border-slate-200 dark:border-slate-800">
        <p className="text-slate-500 text-sm mb-4">
          Powered by Stature.network & AWS Amplify Gen 2
        </p>
        <div className="flex justify-center gap-4">
          <button className="px-6 py-2 bg-blue-600 text-white rounded-full font-semibold hover:bg-blue-700 transition">
            Register Program
          </button>
          <button className="px-6 py-2 bg-slate-200 dark:bg-slate-800 rounded-full font-semibold hover:opacity-80 transition">
            View Protocol Docs
          </button>
        </div>
      </div>
    </div>
  );
}

export function MathSection() {
  // We encode the LaTeX for the URL
  const formula = "S = \\sum (V_{log} \\times D_w \\times T_c)";
  const encodedFormula = encodeURIComponent(formula);

  // Choose a color based on theme (using a filter or different API param)
  // This uses a high-quality SVG renderer
  const mathUrl = `https://latex.codecogs.com/svg.image?{\&space;${encodedFormula}}`;

  const programWeights = [
    {
      name: "TakafulClub (Mobile)",
      weight: 1.0,
      detail: "High frequency, entry-level",
    },
    {
      name: "TakafulClub (Auto)",
      weight: 5.0,
      detail: "Moderate risk, high value",
    },
    { name: "Contract282", weight: 8.0, detail: "Legal integrity & binding" },
  ];

  const industryStandards = [
    { industry: "Micro-Payments", weight: "1.0x", color: "bg-blue-400" },
    { industry: "Social Verification", weight: "2.5x", color: "bg-indigo-400" },
    { industry: "Insurance/Fintech", weight: "5.0x", color: "bg-purple-400" },
    {
      industry: "Legal & Real Estate",
      weight: "8.0x",
      color: "bg-emerald-400",
    },
    { industry: "Institutional Trade", weight: "12.0x", color: "bg-amber-400" },
  ];

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 dark:bg-slate-950 dark:text-slate-100 transition-colors duration-300 font-sans pb-20">

      {/* The Mathematics Section */}
      <div className="px-6 py-12 max-w-4xl mx-auto bg-white dark:bg-slate-900 rounded-3xl border border-slate-200 dark:border-slate-800 shadow-xl overflow-hidden">
        <div className="max-w-4xl mx-auto bg-white dark:bg-slate-900 rounded-3xl border border-slate-200 dark:border-slate-800 shadow-xl overflow-hidden">
          {/* Header with the Equation */}
          {/* We wrap the formula in a standard div. 
              If you have KaTeX installed, you would use a <BlockMath /> component here.
              Otherwise, this styling makes it look like a formal math block. */}

          <div className="bg-slate-100 dark:bg-slate-800 p-8 text-center border-b border-slate-200 dark:border-slate-700">
            <h2 className="text-sm uppercase tracking-widest font-bold text-blue-600 dark:text-blue-400 mb-6">
              The Core Protocol
            </h2>

            <div className="flex justify-center py-4">
              {/* We use an img tag for perfect math rendering */}
              <img
                src={mathUrl}
                alt="Stature Equation"
                className="h-12 md:h-16 dark:invert brightness-200"
              />
            </div>
          </div>

          {/* Breakdown Section */}
          <div className="p-8 space-y-8">
            <div className="flex flex-col md:flex-row gap-6">
              <div className="flex-shrink-0">
                <span className="inline-block px-3 py-1 rounded bg-blue-100 dark:bg-blue-900/40 text-blue-600 font-mono font-bold">
                  V_log
                </span>
              </div>
              <div>
                <h3 className="text-lg font-bold mb-1">Economic Magnitude</h3>
                <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
                  Uses a base-10 log of the transaction value. This ensures a
                  1,000 transaction is worth more than a 10 one, but not 100x
                  more. It prevents "whales" from instantly eclipsing everyone
                  else.
                </p>
              </div>
            </div>

            <div className="flex flex-col md:flex-row gap-6 border-t border-slate-100 dark:border-slate-800 pt-8">
              <div className="flex-shrink-0">
                <span className="inline-block px-3 py-1 rounded bg-emerald-100 dark:bg-emerald-900/40 text-emerald-600 font-mono font-bold">
                  D_w
                </span>
              </div>
              <div>
                <h3 className="text-lg font-bold mb-1">Domain Weight</h3>
                <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
                  This is where you, as the admin, control the "difficulty" or
                  "importance" of a program. Higher-stakes industries (like Auto
                  Insurance) carry more weight than micro-tasks.
                </p>
              </div>
            </div>

            <div className="flex flex-col md:flex-row gap-6 border-t border-slate-100 dark:border-slate-800 pt-8">
              <div className="flex-shrink-0">
                <span className="inline-block px-3 py-1 rounded bg-purple-100 dark:bg-purple-900/40 text-purple-600 font-mono font-bold">
                  T_c
                </span>
              </div>
              <div>
                <h3 className="text-lg font-bold mb-1">Time Consistency</h3>
                <p className="text-slate-600 dark:text-slate-400 leading-relaxed">
                  A small multiplier for how long the user has been active with
                  that program. Trust is a marathon, not a sprint.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Pilot Programs & Industry Standards */}
      <div className="px-6 py-20 max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-12">
        {/* Left: Pilot Companies */}
        <div>
          <h2 className="text-2xl font-bold mb-6 flex items-center gap-2">
            <span className="w-2 h-8 bg-blue-600 rounded-full" />
            Active Pilot Programs
          </h2>
          <div className="space-y-4">
            {programWeights.map((p) => (
              <div
                key={p.name}
                className="flex justify-between items-center p-4 bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800"
              >
                <div>
                  <div className="font-bold">{p.name}</div>
                  <div className="text-xs text-slate-500">{p.detail}</div>
                </div>
                <div className="text-xl font-mono font-bold text-blue-500">
                  x{p.weight.toFixed(1)}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Right: Industry Weights */}
        <div>
          <h2 className="text-2xl font-bold mb-6 flex items-center gap-2">
            <span className="w-2 h-8 bg-emerald-500 rounded-full" />
            Global Industry Standards
          </h2>
          <div className="overflow-hidden rounded-xl border border-slate-200 dark:border-slate-800">
            <table className="w-full text-left bg-white dark:bg-slate-900">
              <thead className="bg-slate-50 dark:bg-slate-800 text-xs uppercase tracking-wider">
                <tr>
                  <th className="px-6 py-3 font-semibold">Industry</th>
                  <th className="px-6 py-3 font-semibold text-right">
                    Multiplier
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
                {industryStandards.map((s) => (
                  <tr key={s.industry}>
                    <td className="px-6 py-4 flex items-center gap-3">
                      <div className={`w-2 h-2 rounded-full ${s.color}`} />
                      <span className="text-sm">{s.industry}</span>
                    </td>
                    <td className="px-6 py-4 text-right font-mono font-bold text-slate-500">
                      {s.weight}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* Progression */}
      <div className="px-6 py-12 max-w-4xl mx-auto text-center">
        <h2 className="text-3xl font-bold mb-4">Level Thresholds</h2>
        <p className="text-slate-500 mb-8 font-mono text-sm">
          Target Level Calculation: 100 × L²
        </p>
        <div className="flex flex-wrap justify-center gap-4">
          {[1, 10, 25, 50, 100].map((lvl) => (
            <div
              key={lvl}
              className="px-4 py-2 bg-slate-200 dark:bg-slate-800 rounded-lg"
            >
              <div className="text-xs text-slate-500">Lvl {lvl}</div>
              <div className="font-bold">
                {(100 * Math.pow(lvl, 2)).toLocaleString()} pts
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
