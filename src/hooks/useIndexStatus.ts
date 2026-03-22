import { useEffect, useRef, useState } from "react";
import { useIndexStore } from "../stores/indexStore";
import { getIndexStatus } from "../services/tauriCommands";

export function useIndexStatus() {
  const { status, setStatus } = useIndexStore();
  const [isLoading, setIsLoading] = useState(false);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    async function fetchStatus() {
      setIsLoading(true);
      try {
        const data = await getIndexStatus();
        setStatus(data);
      } catch (err) {
        console.error("Failed to fetch index status:", err);
      } finally {
        setIsLoading(false);
      }
    }

    fetchStatus();

    intervalRef.current = setInterval(() => {
      fetchStatus();
    }, 2000);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [setStatus]);

  return { status, isLoading };
}
