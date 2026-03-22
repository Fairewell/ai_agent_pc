import { useState, useRef, useCallback, type KeyboardEvent } from "react";
import { Send, Paperclip } from "lucide-react";
import clsx from "clsx";

interface ChatInputProps {
  onSend: (content: string, attachedFiles: string[], useRag: boolean) => void;
  isStreaming: boolean;
}

export function ChatInput({ onSend, isStreaming }: ChatInputProps) {
  const [content, setContent] = useState("");
  const [useRag, setUseRag] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleSend = useCallback(() => {
    const trimmed = content.trim();
    if (!trimmed || isStreaming) return;
    onSend(trimmed, [], useRag);
    setContent("");
    // Reset textarea height
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }
  }, [content, isStreaming, useRag, onSend]);

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleInput = () => {
    const textarea = textareaRef.current;
    if (!textarea) return;
    textarea.style.height = "auto";
    const maxHeight = 6 * 24; // ~6 rows
    textarea.style.height = `${Math.min(textarea.scrollHeight, maxHeight)}px`;
  };

  return (
    <div className="border-t border-gray-200 bg-white px-4 py-3">
      <div className="flex items-end gap-2">
        {/* File attachment placeholder */}
        <button
          type="button"
          className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
          title="Attach files"
        >
          <Paperclip size={18} />
        </button>

        {/* Textarea */}
        <textarea
          ref={textareaRef}
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onKeyDown={handleKeyDown}
          onInput={handleInput}
          placeholder="Type a message..."
          rows={1}
          disabled={isStreaming}
          className={clsx(
            "flex-1 resize-none rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-colors",
            "focus:border-blue-500 focus:ring-1 focus:ring-blue-500",
            "disabled:bg-gray-50 disabled:text-gray-400"
          )}
        />

        {/* Send button */}
        <button
          type="button"
          onClick={handleSend}
          disabled={!content.trim() || isStreaming}
          className={clsx(
            "flex h-10 w-10 shrink-0 items-center justify-center rounded-lg transition-colors",
            content.trim() && !isStreaming
              ? "bg-blue-600 text-white hover:bg-blue-700"
              : "bg-gray-100 text-gray-300"
          )}
        >
          <Send size={18} />
        </button>
      </div>

      {/* RAG toggle */}
      <div className="mt-2 flex items-center gap-2">
        <label className="flex cursor-pointer items-center gap-2 text-xs text-gray-500">
          <input
            type="checkbox"
            checked={useRag}
            onChange={(e) => setUseRag(e.target.checked)}
            className="h-3.5 w-3.5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          Search documents
        </label>
      </div>
    </div>
  );
}
