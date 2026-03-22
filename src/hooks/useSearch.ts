import { useEffect, useRef } from "react";
import { useSearchStore } from "../stores/searchStore";
import { searchFiles } from "../services/tauriCommands";

export function useSearch() {
  const { query, filters, results, isLoading, setResults, setIsLoading } =
    useSearchStore();
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    if (!query.trim()) {
      setResults([]);
      setIsLoading(false);
      return;
    }

    setIsLoading(true);

    debounceRef.current = setTimeout(async () => {
      try {
        const data = await searchFiles(query, filters);
        setResults(data);
      } catch (err) {
        console.error("Search failed:", err);
        setResults([]);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, [query, filters, setResults, setIsLoading]);

  return { results, isLoading };
}
