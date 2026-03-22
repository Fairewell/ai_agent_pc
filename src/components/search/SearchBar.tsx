import { Search } from "lucide-react";
import { useSearchStore } from "../../stores/searchStore";

export function SearchBar() {
  const { query, setQuery } = useSearchStore();

  return (
    <div className="relative">
      <Search
        size={20}
        className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
      />
      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Search documents..."
        className="w-full rounded-xl border border-gray-200 bg-white py-3 pl-12 pr-4 text-lg shadow-sm outline-none transition-shadow focus:border-blue-400 focus:ring-2 focus:ring-blue-100"
      />
      {/* Voice button placeholder */}
    </div>
  );
}
