export interface ProviderConfig {
  id?: number;
  name: string;
  base_url: string;
  api_key: string;
  model_name: string;
  max_tokens: number;
  temperature: number;
  is_default: boolean;
}

export interface ChatSession {
  id?: number;
  title: string;
  provider_id?: number;
  created_at: string;
  updated_at: string;
}

export interface ChatMessage {
  id?: number;
  session_id: number;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  attached_files: string; // JSON array
  tool_calls: string; // JSON array
  created_at: string;
}

export interface TokenPayload {
  session_id: number;
  message_id: number;
  token: string;
}

export interface DonePayload {
  session_id: number;
  message_id: number;
  full_content: string;
  sources: string[];
}

export interface AgentStepPayload {
  session_id: number;
  step: number;
  tool_name: string;
  tool_args: string;
  tool_result_preview: string;
  status: "running" | "completed" | "failed";
}

export interface ErrorPayload {
  session_id: number;
  error: string;
}
