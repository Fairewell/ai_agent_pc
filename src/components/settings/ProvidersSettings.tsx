import { useEffect, useState, useCallback } from "react";
import { Plus, Pencil, Trash2, FlaskConical, Check, X, Loader2 } from "lucide-react";
import clsx from "clsx";
import { useProviders } from "../../hooks/useProviders";
import { ProviderForm } from "./ProviderForm";
import type { ProviderConfig } from "../../types/ai";

type TestState = { status: "idle" | "testing" | "success" | "error"; message?: string };

export function ProvidersSettings() {
  const {
    providers,
    loadProviders,
    addProvider,
    updateProvider,
    deleteProvider,
    testProvider,
  } = useProviders();

  const [editingProvider, setEditingProvider] = useState<ProviderConfig | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [testStates, setTestStates] = useState<Record<number, TestState>>({});

  useEffect(() => {
    loadProviders();
  }, [loadProviders]);

  const handleAdd = () => {
    setEditingProvider(null);
    setShowForm(true);
  };

  const handleEdit = (provider: ProviderConfig) => {
    setEditingProvider(provider);
    setShowForm(true);
  };

  const handleSave = useCallback(
    async (config: ProviderConfig) => {
      if (config.id != null) {
        await updateProvider(config);
      } else {
        await addProvider(config);
      }
      setShowForm(false);
      setEditingProvider(null);
    },
    [addProvider, updateProvider]
  );

  const handleDelete = useCallback(
    async (id: number) => {
      await deleteProvider(id);
    },
    [deleteProvider]
  );

  const handleTest = useCallback(
    async (id: number) => {
      setTestStates((prev) => ({ ...prev, [id]: { status: "testing" } }));
      try {
        const result = await testProvider(id);
        setTestStates((prev) => ({
          ...prev,
          [id]: { status: "success", message: result },
        }));
      } catch (err) {
        setTestStates((prev) => ({
          ...prev,
          [id]: {
            status: "error",
            message: err instanceof Error ? err.message : String(err),
          },
        }));
      }
      // Reset after a few seconds
      setTimeout(() => {
        setTestStates((prev) => ({ ...prev, [id]: { status: "idle" } }));
      }, 4000);
    },
    [testProvider]
  );

  return (
    <div>
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">AI Providers</h2>
        <button
          type="button"
          onClick={handleAdd}
          className="flex items-center gap-1.5 rounded-lg bg-blue-600 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
        >
          <Plus size={16} />
          Add Provider
        </button>
      </div>

      {/* Form (modal-like inline) */}
      {showForm && (
        <div className="mt-6 rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
          <h3 className="mb-4 text-sm font-semibold text-gray-700">
            {editingProvider ? "Edit Provider" : "Add Provider"}
          </h3>
          <ProviderForm
            initial={editingProvider}
            onSave={handleSave}
            onCancel={() => {
              setShowForm(false);
              setEditingProvider(null);
            }}
          />
        </div>
      )}

      {/* Provider cards */}
      <div className="mt-6 space-y-3">
        {providers.length === 0 && !showForm && (
          <div className="rounded-xl border border-dashed border-gray-300 py-12 text-center text-sm text-gray-400">
            No providers configured. Add one to get started.
          </div>
        )}

        {providers.map((provider) => {
          const testState = testStates[provider.id ?? -1] ?? { status: "idle" };

          return (
            <div
              key={provider.id}
              className="flex items-center justify-between rounded-xl border border-gray-200 bg-white px-5 py-4 shadow-sm"
            >
              <div className="min-w-0">
                <div className="flex items-center gap-2">
                  <span className="font-medium text-gray-900">
                    {provider.name}
                  </span>
                  {provider.is_default && (
                    <span className="rounded-full bg-blue-100 px-2 py-0.5 text-[10px] font-semibold uppercase text-blue-700">
                      Default
                    </span>
                  )}
                </div>
                <div className="mt-0.5 text-xs text-gray-500">
                  {provider.model_name} &middot; {provider.base_url}
                </div>
              </div>

              <div className="flex items-center gap-2">
                {/* Test result indicator */}
                {testState.status === "success" && (
                  <span className="text-green-500" title={testState.message}>
                    <Check size={16} />
                  </span>
                )}
                {testState.status === "error" && (
                  <span className="text-red-500" title={testState.message}>
                    <X size={16} />
                  </span>
                )}

                {/* Test button */}
                <button
                  type="button"
                  onClick={() => provider.id != null && handleTest(provider.id)}
                  disabled={testState.status === "testing"}
                  className={clsx(
                    "flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600",
                    testState.status === "testing" && "animate-spin"
                  )}
                  title="Test connection"
                >
                  {testState.status === "testing" ? (
                    <Loader2 size={16} />
                  ) : (
                    <FlaskConical size={16} />
                  )}
                </button>

                {/* Edit button */}
                <button
                  type="button"
                  onClick={() => handleEdit(provider)}
                  className="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
                  title="Edit provider"
                >
                  <Pencil size={16} />
                </button>

                {/* Delete button */}
                <button
                  type="button"
                  onClick={() => provider.id != null && handleDelete(provider.id)}
                  className="flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-red-50 hover:text-red-500"
                  title="Delete provider"
                >
                  <Trash2 size={16} />
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
