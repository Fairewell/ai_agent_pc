import { useCallback } from "react";
import { useProviderStore } from "../stores/providerStore";
import {
  getProviders,
  addProvider as addProviderCmd,
  updateProvider as updateProviderCmd,
  deleteProvider as deleteProviderCmd,
  testProvider as testProviderCmd,
} from "../services/aiCommands";
import type { ProviderConfig } from "../types/ai";

export function useProviders() {
  const providers = useProviderStore((s) => s.providers);
  const activeProvider = useProviderStore((s) => s.activeProvider);
  const setProviders = useProviderStore((s) => s.setProviders);

  const loadProviders = useCallback(async () => {
    const list = await getProviders();
    setProviders(list);
    return list;
  }, [setProviders]);

  const addProvider = useCallback(
    async (config: ProviderConfig) => {
      await addProviderCmd(config);
      await loadProviders();
    },
    [loadProviders]
  );

  const updateProvider = useCallback(
    async (config: ProviderConfig) => {
      await updateProviderCmd(config);
      await loadProviders();
    },
    [loadProviders]
  );

  const deleteProvider = useCallback(
    async (id: number) => {
      await deleteProviderCmd(id);
      await loadProviders();
    },
    [loadProviders]
  );

  const testProvider = useCallback(async (id: number): Promise<string> => {
    return testProviderCmd(id);
  }, []);

  return {
    providers,
    activeProvider,
    loadProviders,
    addProvider,
    updateProvider,
    deleteProvider,
    testProvider,
  };
}
