import { ProvidersSettings } from "../../components/settings/ProvidersSettings";

export function SettingsPage() {
  return (
    <div className="mx-auto max-w-3xl px-6 py-8">
      <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
      <div className="mt-8">
        <ProvidersSettings />
      </div>
    </div>
  );
}
