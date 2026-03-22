import { useState, type KeyboardEvent } from "react";
import { useNavigate } from "react-router-dom";
import { Sparkles } from "lucide-react";
import { useChatStore } from "../../stores/chatStore";
import { createChatSession, sendChatMessage } from "../../services/aiCommands";

export function PromptBar() {
  const [prompt, setPrompt] = useState("");
  const [isSending, setIsSending] = useState(false);
  const navigate = useNavigate();
  const setActiveSession = useChatStore((s) => s.setActiveSession);
  const addMessage = useChatStore((s) => s.addMessage);
  const clearStreaming = useChatStore((s) => s.clearStreaming);
  const setMessages = useChatStore((s) => s.setMessages);

  const handleSubmit = async () => {
    const trimmed = prompt.trim();
    if (!trimmed || isSending) return;

    setIsSending(true);
    try {
      // Create a new session
      const session = await createChatSession(
        trimmed.length > 60 ? trimmed.slice(0, 60) + "..." : trimmed
      );
      const sessionId = session.id ?? 0;

      // Set up chat store state
      setActiveSession(sessionId);
      setMessages([]);
      clearStreaming();

      // Add user message to local state
      addMessage({
        session_id: sessionId,
        role: "user",
        content: trimmed,
        attached_files: "[]",
        tool_calls: "[]",
        created_at: new Date().toISOString(),
      });

      // Navigate to chat page
      navigate("/chat");

      // Send message with RAG enabled
      await sendChatMessage(sessionId, trimmed, [], true);
    } catch (err) {
      console.error("PromptBar error:", err);
    } finally {
      setIsSending(false);
      setPrompt("");
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="relative">
      <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
        <Sparkles size={16} className="text-purple-400" />
      </div>
      <input
        type="text"
        value={prompt}
        onChange={(e) => setPrompt(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Ask AI about your documents..."
        disabled={isSending}
        className="w-full rounded-lg border border-gray-200 bg-white py-2.5 pl-10 pr-4 text-sm text-gray-700 outline-none transition-colors placeholder:text-gray-400 focus:border-purple-400 focus:ring-1 focus:ring-purple-400 disabled:bg-gray-50 disabled:text-gray-400"
      />
    </div>
  );
}
