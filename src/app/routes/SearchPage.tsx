import { SearchBar } from "../../components/search/SearchBar";
import { SearchResults } from "../../components/search/SearchResults";
import { PromptBar } from "../../components/search/PromptBar";
import { useSearch } from "../../hooks/useSearch";

export function SearchPage() {
  const { results, isLoading } = useSearch();

  return (
    <div className="mx-auto max-w-3xl px-6 py-8">
      <SearchBar />
      <div className="mt-3">
        <PromptBar />
      </div>
      <div className="mt-6">
        <SearchResults results={results} isLoading={isLoading} />
      </div>
    </div>
  );
}
