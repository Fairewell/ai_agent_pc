import { useEffect, useRef } from "react";
import { useChat } from "../../hooks/useChat";
import { ChatSessionList } from "../../components/chat/ChatSessionList";
import { ChatMessageBubble } from "../../components/chat/ChatMessageBubble";
import { ChatInput } from "../../components/chat/ChatInput";
import { StreamingIndicator } from "../../components/chat/StreamingIndicator";
import { MessageSquare } from "lucide-react";
import clsx from "clsx";

export function ChatPage() {
  const {
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
  } = useChat();

  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Load sessions on mount
  useEffect(() => {
    loadSessions();
  }, [loadSessions]);

  // Auto-scroll to bottom when messages change or streaming content updates
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, streamingContent]);

  const handleSend = (
    content: string,
    attachedFiles: string[],
    useRag: boolean
  ) => {
    sendMessage(content, attachedFiles, useRag);
  };

  return (
    <div className="flex h-full">
      {/* Session sidebar */}
      <div className="w-64 shrink-0">
        <ChatSessionList
          sessions={sessions}
          activeSessionId={activeSessionId}
          onSelect={loadSession}
          onCreate={createSession}
          onDelete={deleteSession}
        />
      </div>

      {/* Chat area */}
      <div className="flex flex-1 flex-col">
        {activeSessionId == null ? (
          /* Empty state */
          <div className="flex flex-1 flex-col items-center justify-center text-gray-400">
            <MessageSquare size={48} strokeWidth={1.5} />
            <p className="mt-4 text-lg">Select a chat or start a new one</p>
            <button
              type="button"
              onClick={() => createSession()}
              className="mt-4 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
            >
              New Chat
            </button>
          </div>
        ) : (
          <>
            {/* Messages */}
            <div className="flex-1 overflow-y-auto px-6 py-4">
              {messages.length === 0 && !isStreaming && (
                <div className="flex h-full items-center justify-center text-sm text-gray-400">
                  Send a message to start the conversation
                </div>
              )}

              <div className="mx-auto max-w-3xl space-y-4">
                {messages.map((msg, idx) => {
                  // Show sources on the last assistant message
                  const isLastAssistant =
                    msg.role === "assistant" &&
                    !messages.slice(idx + 1).some((m) => m.role === "assistant");
                  return (
                    <ChatMessageBubble
                      key={msg.id ?? `msg-${idx}`}
                      message={msg}
                      sources={isLastAssistant ? sources : undefined}
                    />
                  );
                })}

                {/* Streaming partial message */}
                {isStreaming && streamingContent && (
                  <div className="flex justify-start">
                    <div
                      className={clsx(
                        "max-w-[80%] rounded-lg border border-gray-200 bg-white px-4 py-3 text-gray-900"
                      )}
                    >
                      <div className="whitespace-pre-wrap text-sm">
                        {streamingContent}
                        <span className="ml-0.5 inline-block h-4 w-0.5 animate-pulse bg-gray-400" />
                      </div>
                    </div>
                  </div>
                )}

                {/* Streaming indicator (no content yet) */}
                {isStreaming && !streamingContent && (
                  <div className="flex justify-start">
                    <div className="rounded-lg border border-gray-200 bg-white px-4 py-3">
                      <StreamingIndicator />
                    </div>
                  </div>
                )}

                <div ref={messagesEndRef} />
              </div>
            </div>

            {/* Input */}
            <ChatInput onSend={handleSend} isStreaming={isStreaming} />
          </>
        )}
      </div>
    </div>
  );
}
