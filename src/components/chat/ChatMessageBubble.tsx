import clsx from "clsx";
import { format } from "date-fns";
import type { ChatMessage } from "../../types/ai";

interface ChatMessageBubbleProps {
  message: ChatMessage;
  sources?: string[];
}

export function ChatMessageBubble({ message, sources }: ChatMessageBubbleProps) {
  const isUser = message.role === "user";
  const isTool = message.role === "tool";
  const timestamp = message.created_at
    ? format(new Date(message.created_at), "MMM d, h:mm a")
    : "";

  if (isTool) {
    let toolName = "Tool";
    try {
      const calls = JSON.parse(message.tool_calls || "[]");
      if (calls.length > 0) toolName = calls[0].name ?? "Tool";
    } catch {
      // ignore
    }

    return (
      <div className="flex justify-start">
        <div className="max-w-[80%] rounded-lg border border-gray-200 bg-gray-100 px-4 py-3">
          <div className="mb-1 text-xs font-semibold text-gray-500">
            {toolName}
          </div>
          <pre className="whitespace-pre-wrap font-mono text-xs text-gray-700">
            {message.content.length > 300
              ? message.content.slice(0, 300) + "..."
              : message.content}
          </pre>
          {timestamp && (
            <div className="mt-2 text-[10px] text-gray-400">{timestamp}</div>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className={clsx("flex", isUser ? "justify-end" : "justify-start")}>
      <div
        className={clsx(
          "max-w-[80%] rounded-lg px-4 py-3",
          isUser
            ? "bg-blue-600 text-white"
            : "border border-gray-200 bg-white text-gray-900"
        )}
      >
        <div className="whitespace-pre-wrap text-sm">{message.content}</div>

        {/* Sources */}
        {!isUser && sources && sources.length > 0 && (
          <div className="mt-3 border-t border-gray-200 pt-2">
            <div className="mb-1 text-[10px] font-semibold uppercase tracking-wider text-gray-400">
              Sources
            </div>
            <ul className="space-y-0.5">
              {sources.map((src, i) => (
                <li key={i} className="truncate text-xs text-blue-500">
                  {src}
                </li>
              ))}
            </ul>
          </div>
        )}

        {timestamp && (
          <div
            className={clsx(
              "mt-2 text-[10px]",
              isUser ? "text-blue-200" : "text-gray-400"
            )}
          >
            {timestamp}
          </div>
        )}
      </div>
    </div>
  );
}
