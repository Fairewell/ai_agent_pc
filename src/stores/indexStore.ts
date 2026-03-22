import { create } from "zustand";
import type { IndexStatus } from "../types/search";

interface IndexState {
  status: IndexStatus;
  scanPath: string;
  setStatus: (status: IndexStatus) => void;
  setScanPath: (scanPath: string) => void;
}

export const useIndexStore = create<IndexState>((set) => ({
  status: {
    total_files: 0,
    indexed_files: 0,
    is_scanning: false,
  },
  scanPath: "",
  setStatus: (status) => set({ status }),
  setScanPath: (scanPath) => set({ scanPath }),
}));
