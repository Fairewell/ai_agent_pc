import { useCallback } from "react";
import { useChatStore } from "../stores/chatStore";
import { useStreamingResponse } from "./useStreamingResponse";
import {
  sendChatMessage,
  createChatSession,
  listChatSessions,
  deleteChatSession,
  getChatMessages,
} from "../services/aiCommands";
import type { ChatMessage } from "../types/ai";

export function useChat() {
  const sessions = useChatStore((s) => s.sessions);
  const activeSessionId = useChatStore((s) => s.activeSessionId);
  const messages = useChatStore((s) => s.messages);
  const isStreaming = useChatStore((s) => s.isStreaming);
  const streamingContent = useChatStore((s) => s.streamingContent);
  const sources = useChatStore((s) => s.sources);

  const setSessions = useChatStore((s) => s.setSessions);
  const setActiveSession = useChatStore((s) => s.setActiveSession);
  const setMessages = useChatStore((s) => s.setMessages);
  const addMessage = useChatStore((s) => s.addMessage);
  const clearStreaming = useChatStore((s) => s.clearStreaming);

  // Set up streaming listeners for active session
  useStreamingResponse(activeSessionId);

  const loadSessions = useCallback(async () => {
    const list = await listChatSessions();
    setSessions(list);
    return list;
  }, [setSessions]);

  const createSession = useCallback(
    async (title?: string) => {
      const session = await createChatSession(
        title ?? `Chat ${new Date().toLocaleString()}`
      );
      await loadSessions();
      setActiveSession(session.id ?? null);
      setMessages([]);
      clearStreaming();
      return session;
    },
    [loadSessions, setActiveSession, setMessages, clearStreaming]
  );

  const loadSession = useCallback(
    async (id: number) => {
      setActiveSession(id);
      clearStreaming();
      const msgs = await getChatMessages(id);
      setMessages(msgs);
    },
    [setActiveSession, setMessages, clearStreaming]
  );

  const deleteSession = useCallback(
    async (id: number) => {
      await deleteChatSession(id);
      const list = await loadSessions();
      if (activeSessionId === id) {
        const next = list[0];
        if (next?.id != null) {
          await loadSession(next.id);
        } else {
          setActiveSession(null);
          setMessages([]);
        }
      }
    },
    [activeSessionId, loadSessions, loadSession, setActiveSession, setMessages]
  );

  const sendMessage = useCallback(
    async (
      content: string,
      attachedFiles: string[] = [],
      useRag: boolean = false
    ) => {
      if (activeSessionId == null) return;

      // Add user message to local state immediately
      const userMessage: ChatMessage = {
        session_id: activeSessionId,
        role: "user",
        content,
        attached_files: JSON.stringify(attachedFiles),
        tool_calls: "[]",
        created_at: new Date().toISOString(),
      };
      addMessage(userMessage);
      clearStreaming();

      // Send to backend — streaming events will update the store
      await sendChatMessage(activeSessionId, content, attachedFiles, useRag);
    },
    [activeSessionId, addMessage, clearStreaming]
  );

  return {
    sessions,
    activeSessionId,
    messages,
    isStreaming,
    streamingContent,
    sources,
    sendMessage,
    createSession,
    loadSession,
    loadSessions,
    deleteSession,
  };
}
