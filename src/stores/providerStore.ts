import { create } from "zustand";
import type { ProviderConfig } from "../types/ai";

interface ProviderState {
  providers: ProviderConfig[];
  activeProvider: ProviderConfig | null;
}

interface ProviderActions {
  setProviders: (providers: ProviderConfig[]) => void;
  setActiveProvider: (provider: ProviderConfig | null) => void;
  addProvider: (provider: ProviderConfig) => void;
  removeProvider: (id: number) => void;
  updateProvider: (provider: ProviderConfig) => void;
}

export const useProviderStore = create<ProviderState & ProviderActions>(
  (set) => ({
    providers: [],
    activeProvider: null,

    setProviders: (providers) => {
      const defaultProvider = providers.find((p) => p.is_default) ?? null;
      set({ providers, activeProvider: defaultProvider });
    },

    setActiveProvider: (provider) => set({ activeProvider: provider }),

    addProvider: (provider) =>
      set((state) => ({ providers: [...state.providers, provider] })),

    removeProvider: (id) =>
      set((state) => ({
        providers: state.providers.filter((p) => p.id !== id),
        activeProvider:
          state.activeProvider?.id === id ? null : state.activeProvider,
      })),

    updateProvider: (provider) =>
      set((state) => ({
        providers: state.providers.map((p) =>
          p.id === provider.id ? provider : p
        ),
        activeProvider:
          state.activeProvider?.id === provider.id
            ? provider
            : state.activeProvider,
      })),
  })
);
