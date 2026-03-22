import { Virtuoso } from "react-virtuoso";
import { ResultCard } from "./ResultCard";
import type { SearchResult } from "../../types/search";

interface SearchResultsProps {
  results: SearchResult[];
  isLoading: boolean;
}

export function SearchResults({ results, isLoading }: SearchResultsProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-16">
        <div className="h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-500" />
      </div>
    );
  }

  if (results.length === 0) {
    return (
      <div className="py-16 text-center text-gray-400">
        <p className="text-lg">No results</p>
        <p className="mt-1 text-sm">Try searching for a document</p>
      </div>
    );
  }

  return (
    <Virtuoso
      style={{ height: "calc(100vh - 160px)" }}
      data={results}
      itemContent={(_index, item) => (
        <div className="py-1">
          <ResultCard result={item} />
        </div>
      )}
    />
  );
}
