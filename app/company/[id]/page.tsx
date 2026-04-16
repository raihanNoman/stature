import React from 'react';
import { 
  ShieldCheck, 
  Users, 
  BarChart3, 
  Zap, 
  History, 
  CheckCircle2, 
  AlertCircle 
} from 'lucide-react';

// --- Types ---
interface RecordLog {
  id: string;
  userName: string;
  userAddress: string;
  amount: number;
  type: 'positive' | 'negative';
  timestamp: number;
}

export default function CompanyPage() {
  // Dummy Data - In production, fetch from Company account + Amplify
  const companyData = {
    name: "DevShop Delta",
    rep: "7xWp...3u9Y",
    stature: 4500,
    weight: 20, // 2x multiplier
    isVerified: true,
    totalPositive: 840,
    totalNegative: 12,
    recordCount: 852,
    maxCap: 5000,
  };

  const recentLogs: RecordLog[] = [
    { id: '1', userName: 'Alice.sol', userAddress: 'A1b2...c3d4', amount: 15, type: 'positive', timestamp: Date.now() - 3600000 },
    { id: '2', userName: 'Bob.sol', userAddress: 'B9z8...y7x6', amount: 12, type: 'positive', timestamp: Date.now() - 7200000 },
    { id: '3', userName: 'Charlie.sol', userAddress: 'C5v4...n3m2', amount: -5, type: 'negative', timestamp: Date.now() - 86400000 },
  ];

  const healthScore = Math.round((companyData.totalPositive / companyData.recordCount) * 100);

  return (
    <div className="min-h-screen bg-zinc-50 text-zinc-900 dark:bg-black dark:text-zinc-100 p-4 md:p-8">
      <div className="max-w-6xl mx-auto">
        
        {/* Company Header */}
        <div className="flex flex-col md:flex-row justify-between items-start gap-8 mb-12">
          <div className="flex items-center gap-6">
            <div className="w-24 h-24 rounded-3xl bg-zinc-200 dark:bg-zinc-800 flex items-center justify-center border border-zinc-300 dark:border-zinc-700 shadow-xl">
              <Zap size={40} className="text-yellow-500" />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-4xl font-black tracking-tight uppercase">{companyData.name}</h1>
                {companyData.isVerified && <ShieldCheck className="text-blue-500" size={24} />}
              </div>
              <p className="font-mono text-zinc-500 text-sm mt-1">OPERATOR: {companyData.rep}</p>
              <div className="flex gap-2 mt-3">
                <span className="px-2 py-1 rounded-md bg-emerald-100 dark:bg-emerald-900/30 text-emerald-600 text-[10px] font-bold uppercase tracking-widest">
                  {companyData.weight / 10}x Weight
                </span>
                <span className="px-2 py-1 rounded-md bg-zinc-200 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 text-[10px] font-bold uppercase tracking-widest">
                  {companyData.recordCount} / {companyData.maxCap} Records
                </span>
              </div>
            </div>
          </div>

          <div className="w-full md:w-auto p-6 rounded-3xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 flex flex-col items-center">
            <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest mb-1">Company Stature</span>
            <span className="text-5xl font-black tracking-tighter text-emerald-500">{companyData.stature}</span>
          </div>
        </div>

        {/* Analytics Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          <MetricCard 
            label="Reputation Integrity" 
            value={`${healthScore}%`} 
            icon={<BarChart3 size={20} />} 
            subText={`${companyData.totalPositive} Positive vs ${companyData.totalNegative} Negative`}
          />
          <MetricCard 
            label="Total Contributors" 
            value={companyData.recordCount.toLocaleString()} 
            icon={<Users size={20} />} 
            subText="Unique wallet interactions"
          />
          <MetricCard 
            label="Current Capacity" 
            value={`${Math.round((companyData.recordCount / companyData.maxCap) * 100)}%`} 
            icon={<History size={20} />} 
            subText="Remaining record mints available"
          />
        </div>

        {/* Live Stature Feed */}
        <div className="rounded-3xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900/50 overflow-hidden">
          <div className="p-6 border-b border-zinc-200 dark:border-zinc-800 flex justify-between items-center">
            <h2 className="font-bold flex items-center gap-2 uppercase tracking-tight">
              <History size={18} className="text-zinc-400" /> Recent Network Activity
            </h2>
            <button className="text-xs font-bold text-blue-500 hover:underline">Export CSV</button>
          </div>
          
          <div className="divide-y divide-zinc-200 dark:divide-zinc-800">
            {recentLogs.map((log) => (
              <div key={log.id} className="p-4 flex items-center justify-between hover:bg-zinc-100 dark:hover:bg-zinc-800/50 transition-colors">
                <div className="flex items-center gap-4">
                  {log.type === 'positive' ? 
                    <CheckCircle2 size={20} className="text-emerald-500" /> : 
                    <AlertCircle size={20} className="text-red-500" />
                  }
                  <div>
                    <p className="font-bold text-sm">{log.userName}</p>
                    <p className="text-[10px] font-mono text-zinc-500">{log.userAddress}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className={`font-black ${log.type === 'positive' ? 'text-emerald-500' : 'text-red-500'}`}>
                    {log.type === 'positive' ? '+' : ''}{log.amount}
                  </p>
                  <p className="text-[10px] text-zinc-400 uppercase">{new Date(log.timestamp).toLocaleTimeString()}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

      </div>
    </div>
  );
}

// --- Helper Components ---

function MetricCard({ label, value, icon, subText }: { label: string, value: string, icon: React.ReactNode, subText: string }) {
  return (
    <div className="p-6 rounded-3xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800">
      <div className="flex items-center gap-2 text-zinc-400 mb-4">
        {icon}
        <span className="text-[10px] font-bold uppercase tracking-widest">{label}</span>
      </div>
      <p className="text-3xl font-black mb-1">{value}</p>
      <p className="text-xs text-zinc-500">{subText}</p>
    </div>
  );
}