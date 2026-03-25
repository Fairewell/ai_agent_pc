import { useState, useEffect } from "react";
import type { ProviderConfig } from "../../types/ai";

interface ProviderFormProps {
  initial?: ProviderConfig | null;
  onSave: (config: ProviderConfig) => void;
  onCancel: () => void;
}

const PRESETS: Record<string, { base_url: string; model_name: string }> = {
  OpenAI: { base_url: "https://api.openai.com", model_name: "gpt-4o" },
  Ollama: { base_url: "http://localhost:11434", model_name: "llama3" },
  OpenRouter: {
    base_url: "https://openrouter.ai/api",
    model_name: "openai/gpt-4o",
  },
};

const emptyConfig: ProviderConfig = {
  name: "",
  base_url: "",
  api_key: "",
  model_name: "",
  max_tokens: 4096,
  temperature: 0.7,
  is_default: false,
};

export function ProviderForm({ initial, onSave, onCancel }: ProviderFormProps) {
  const [form, setForm] = useState<ProviderConfig>(initial ?? emptyConfig);

  useEffect(() => {
    setForm(initial ?? emptyConfig);
  }, [initial]);

  const set = <K extends keyof ProviderConfig>(
    key: K,
    value: ProviderConfig[K]
  ) => {
    setForm((prev) => ({ ...prev, [key]: value }));
  };

  const applyPreset = (name: string) => {
    const preset = PRESETS[name];
    if (!preset) return;
    setForm((prev) => ({
      ...prev,
      name: prev.name || name,
      base_url: preset.base_url,
      model_name: prev.model_name || preset.model_name,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(form);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      {/* Presets */}
      <div>
        <label className="mb-1.5 block text-xs font-medium text-gray-500">
          Quick presets
        </label>
        <div className="flex gap-2">
          {Object.keys(PRESETS).map((name) => (
            <button
              key={name}
              type="button"
              onClick={() => applyPreset(name)}
              className="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-xs font-medium text-gray-700 transition-colors hover:bg-gray-50"
            >
              {name}
            </button>
          ))}
        </div>
      </div>

      {/* Name */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          Name
        </label>
        <input
          type="text"
          value={form.name}
          onChange={(e) => set("name", e.target.value)}
          placeholder="My Provider"
          required
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
        />
      </div>

      {/* Base URL */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          Base URL
        </label>
        <input
          type="url"
          value={form.base_url}
          onChange={(e) => set("base_url", e.target.value)}
          placeholder="https://api.openai.com"
          required
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
        />
      </div>

      {/* API Key */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          API Key
        </label>
        <input
          type="password"
          value={form.api_key}
          onChange={(e) => set("api_key", e.target.value)}
          placeholder="sk-..."
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
        />
      </div>

      {/* Model Name */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          Model Name
        </label>
        <input
          type="text"
          value={form.model_name}
          onChange={(e) => set("model_name", e.target.value)}
          placeholder="gpt-4o"
          required
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
        />
      </div>

      {/* Max Tokens */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          Max Tokens
        </label>
        <input
          type="number"
          value={form.max_tokens}
          onChange={(e) => set("max_tokens", Number(e.target.value))}
          min={1}
          max={128000}
          className="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
        />
      </div>

      {/* Temperature */}
      <div>
        <label className="mb-1.5 block text-sm font-medium text-gray-700">
          Temperature: {form.temperature.toFixed(2)}
        </label>
        <input
          type="range"
          value={form.temperature}
          onChange={(e) => set("temperature", Number(e.target.value))}
          min={0}
          max={2}
          step={0.05}
          className="w-full accent-blue-600"
        />
        <div className="flex justify-between text-[10px] text-gray-400">
          <span>0 (Precise)</span>
          <span>2 (Creative)</span>
        </div>
      </div>

      {/* Is Default */}
      <label className="flex items-center gap-2 text-sm text-gray-700">
        <input
          type="checkbox"
          checked={form.is_default}
          onChange={(e) => set("is_default", e.target.checked)}
          className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        />
        Set as default provider
      </label>

      {/* Actions */}
      <div className="flex justify-end gap-3 pt-2">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
        >
          Cancel
        </button>
        <button
          type="submit"
          className="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
        >
          Save
        </button>
      </div>
    </form>
  );
}
