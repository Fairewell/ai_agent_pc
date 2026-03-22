import { Settings } from "lucide-react";

export function SettingsPage() {
  return (
    <div className="mx-auto max-w-3xl px-6 py-8">
      <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
      <div className="mt-8 flex flex-col items-center justify-center py-16 text-gray-400">
        <Settings size={48} strokeWidth={1.5} />
        <p className="mt-4 text-lg">Settings coming soon</p>
      </div>
    </div>
  );
}
