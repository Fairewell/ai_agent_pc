import { useIndexStatus } from "../../hooks/useIndexStatus";
import { startScan } from "../../services/tauriCommands";
import { Database, HardDrive, Loader2 } from "lucide-react";

export function DashboardPage() {
  const { status, isLoading } = useIndexStatus();

  const handleStartScan = () => {
    startScan("/").catch((err) =>
      console.error("Failed to start scan:", err)
    );
  };

  return (
    <div className="mx-auto max-w-3xl px-6 py-8">
      <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>

      <div className="mt-6 grid gap-4 sm:grid-cols-3">
        {/* Total files */}
        <div className="rounded-lg border border-gray-200 bg-white p-5">
          <div className="flex items-center gap-2 text-gray-500">
            <HardDrive size={16} />
            <span className="text-sm font-medium">Total Files</span>
          </div>
          <p className="mt-2 text-2xl font-bold text-gray-900">
            {status.total_files.toLocaleString()}
          </p>
        </div>

        {/* Indexed files */}
        <div className="rounded-lg border border-gray-200 bg-white p-5">
          <div className="flex items-center gap-2 text-gray-500">
            <Database size={16} />
            <span className="text-sm font-medium">Indexed</span>
          </div>
          <p className="mt-2 text-2xl font-bold text-gray-900">
            {status.indexed_files.toLocaleString()}
          </p>
        </div>

        {/* Scan status */}
        <div className="rounded-lg border border-gray-200 bg-white p-5">
          <div className="flex items-center gap-2 text-gray-500">
            <Loader2
              size={16}
              className={status.is_scanning ? "animate-spin" : ""}
            />
            <span className="text-sm font-medium">Status</span>
          </div>
          <p className="mt-2 text-lg font-semibold text-gray-900">
            {status.is_scanning ? "Scanning..." : "Idle"}
          </p>
        </div>
      </div>

      <div className="mt-8">
        <button
          onClick={handleStartScan}
          disabled={status.is_scanning || isLoading}
          className="rounded-lg bg-blue-600 px-5 py-2.5 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {status.is_scanning ? "Scanning..." : "Start Scan"}
        </button>
      </div>
    </div>
  );
}
