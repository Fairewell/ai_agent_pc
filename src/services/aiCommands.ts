import { invoke } from "@tauri-apps/api/core";
import type { ProviderConfig, ChatSession, ChatMessage } from "../types/ai";

// ── Provider CRUD ──────────────────────────────────────────────

export async function getProviders(): Promise<ProviderConfig[]> {
  return invoke<ProviderConfig[]>("get_providers");
}

export async function addProvider(config: ProviderConfig): Promise<ProviderConfig> {
  return invoke<ProviderConfig>("add_provider", { config });
}

export async function updateProvider(config: ProviderConfig): Promise<ProviderConfig> {
  return invoke<ProviderConfig>("update_provider", { config });
}

export async function deleteProvider(id: number): Promise<void> {
  return invoke<void>("delete_provider", { id });
}

export async function testProvider(id: number): Promise<string> {
  return invoke<string>("test_provider", { id });
}

// ── Chat Sessions ──────────────────────────────────────────────

export async function createChatSession(title: string): Promise<ChatSession> {
  return invoke<ChatSession>("create_chat_session", { title });
}

export async function listChatSessions(): Promise<ChatSession[]> {
  return invoke<ChatSession[]>("list_chat_sessions");
}

export async function deleteChatSession(id: number): Promise<void> {
  return invoke<void>("delete_chat_session", { id });
}

export async function getChatMessages(sessionId: number): Promise<ChatMessage[]> {
  return invoke<ChatMessage[]>("get_chat_messages", { sessionId });
}

// ── Messaging ──────────────────────────────────────────────────

export async function sendChatMessage(
  sessionId: number,
  message: string,
  attachedFiles: string[],
  useRag: boolean
): Promise<void> {
  return invoke<void>("send_chat_message", {
    sessionId,
    message,
    attachedFiles,
    useRag,
  });
}

// ── Transform ──────────────────────────────────────────────────

export async function transformDocument(
  path: string,
  prompt: string
): Promise<string> {
  return invoke<string>("transform_document", { path, prompt });
}

// ── Agent ──────────────────────────────────────────────────────

export async function agentExecute(
  prompt: string,
  sessionId: number
): Promise<void> {
  return invoke<void>("agent_execute", { prompt, sessionId });
}
