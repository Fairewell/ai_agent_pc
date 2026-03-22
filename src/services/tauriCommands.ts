import { invoke } from "@tauri-apps/api/core";
import type { SearchResult, SearchFilters, IndexStatus, FileDetail } from "../types/search";

export async function searchFiles(
  query: string,
  filters?: SearchFilters
): Promise<SearchResult[]> {
  return invoke<SearchResult[]>("search_files", { query, filters });
}

export async function getIndexStatus(): Promise<IndexStatus> {
  return invoke<IndexStatus>("get_index_status");
}

export async function startScan(path: string): Promise<void> {
  return invoke<void>("start_scan", { path });
}

export async function openFile(path: string): Promise<void> {
  return invoke<void>("open_file", { path });
}

export async function getFileDetail(path: string): Promise<FileDetail> {
  return invoke<FileDetail>("get_file_detail", { path });
}
