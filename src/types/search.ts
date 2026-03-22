export interface SearchResult {
  id: number;
  path: string;
  filename: string;
  extension: string;
  size_bytes: number;
  modified_at: string;
  snippet: string;
}

export interface SearchFilters {
  extensions?: string[];
  drive?: string;
  max_results?: number;
}

export interface IndexStatus {
  total_files: number;
  indexed_files: number;
  is_scanning: boolean;
}

export interface FileDetail {
  id: number;
  path: string;
  filename: string;
  extension: string;
  size_bytes: number;
  created_at: string;
  modified_at: string;
  indexed_at: string;
  drive_letter: string;
  drive_type: string;
  mime_type: string;
  index_status: string;
  metadata: Record<string, string>;
}
