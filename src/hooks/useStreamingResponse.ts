import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useChatStore } from "../stores/chatStore";
import type { TokenPayload, DonePayload, ErrorPayload } from "../types/ai";

/**
 * Listens to Tauri streaming events for the active chat session.
 * Filters events by sessionId so only the active session is updated.
 */
export function useStreamingResponse(sessionId: number | null) {
  const appendStreamingToken = useChatStore((s) => s.appendStreamingToken);
  const finalizeStreaming = useChatStore((s) => s.finalizeStreaming);
  const clearStreaming = useChatStore((s) => s.clearStreaming);

  useEffect(() => {
    if (sessionId == null) return;

    const unlisteners: Array<() => void> = [];

    async function setup() {
      const unToken = await listen<TokenPayload>("ai:token", (event) => {
        if (event.payload.session_id === sessionId) {
          appendStreamingToken(event.payload.token);
        }
      });
      unlisteners.push(unToken);

      const unDone = await listen<DonePayload>("ai:done", (event) => {
        if (event.payload.session_id === sessionId) {
          finalizeStreaming(
            event.payload.full_content,
            event.payload.sources
          );
        }
      });
      unlisteners.push(unDone);

      const unError = await listen<ErrorPayload>("ai:error", (event) => {
        if (event.payload.session_id === sessionId) {
          clearStreaming();
          console.error("AI streaming error:", event.payload.error);
        }
      });
      unlisteners.push(unError);

      // Optional: agent step events for future progress UI
      const unStep = await listen("ai:agent_step", () => {
        // Could update agent progress state here
      });
      unlisteners.push(unStep);
    }

    setup();

    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [sessionId, appendStreamingToken, finalizeStreaming, clearStreaming]);
}
