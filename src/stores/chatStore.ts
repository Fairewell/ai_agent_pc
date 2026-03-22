import { create } from "zustand";
import type { ChatSession, ChatMessage } from "../types/ai";

interface ChatState {
  sessions: ChatSession[];
  activeSessionId: number | null;
  messages: ChatMessage[];
  streamingContent: string;
  isStreaming: boolean;
  sources: string[];
}

interface ChatActions {
  setSessions: (sessions: ChatSession[]) => void;
  setActiveSession: (id: number | null) => void;
  addMessage: (message: ChatMessage) => void;
  setMessages: (messages: ChatMessage[]) => void;
  appendStreamingToken: (token: string) => void;
  finalizeStreaming: (fullContent: string, sources: string[]) => void;
  clearStreaming: () => void;
}

export const useChatStore = create<ChatState & ChatActions>((set) => ({
  sessions: [],
  activeSessionId: null,
  messages: [],
  streamingContent: "",
  isStreaming: false,
  sources: [],

  setSessions: (sessions) => set({ sessions }),

  setActiveSession: (id) => set({ activeSessionId: id }),

  addMessage: (message) =>
    set((state) => ({ messages: [...state.messages, message] })),

  setMessages: (messages) => set({ messages }),

  appendStreamingToken: (token) =>
    set((state) => ({
      streamingContent: state.streamingContent + token,
      isStreaming: true,
    })),

  finalizeStreaming: (fullContent, sources) =>
    set((state) => ({
      streamingContent: "",
      isStreaming: false,
      sources,
      messages: [
        ...state.messages,
        {
          session_id: state.activeSessionId ?? 0,
          role: "assistant" as const,
          content: fullContent,
          attached_files: "[]",
          tool_calls: "[]",
          created_at: new Date().toISOString(),
        },
      ],
    })),

  clearStreaming: () =>
    set({ streamingContent: "", isStreaming: false, sources: [] }),
}));
