import { create } from "zustand";
import type { SearchResult, SearchFilters } from "../types/search";

interface SearchState {
  query: string;
  results: SearchResult[];
  isLoading: boolean;
  filters: SearchFilters;
  setQuery: (query: string) => void;
  setResults: (results: SearchResult[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  setFilters: (filters: SearchFilters) => void;
  clearResults: () => void;
}

export const useSearchStore = create<SearchState>((set) => ({
  query: "",
  results: [],
  isLoading: false,
  filters: {},
  setQuery: (query) => set({ query }),
  setResults: (results) => set({ results }),
  setIsLoading: (isLoading) => set({ isLoading }),
  setFilters: (filters) => set({ filters }),
  clearResults: () => set({ results: [], query: "" }),
}));
