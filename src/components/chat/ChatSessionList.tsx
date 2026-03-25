import { Plus, X } from "lucide-react";
import { format } from "date-fns";
import clsx from "clsx";
import type { ChatSession } from "../../types/ai";

interface ChatSessionListProps {
  sessions: ChatSession[];
  activeSessionId: number | null;
  onSelect: (id: number) => void;
  onCreate: () => void;
  onDelete: (id: number) => void;
}

export function ChatSessionList({
  sessions,
  activeSessionId,
  onSelect,
  onCreate,
  onDelete,
}: ChatSessionListProps) {
  return (
    <div className="flex h-full flex-col border-r border-gray-200 bg-gray-50">
      {/* Header */}
      <div className="flex items-center justify-between border-b border-gray-200 px-4 py-3">
        <h2 className="text-sm font-semibold text-gray-700">Chats</h2>
        <button
          type="button"
          onClick={onCreate}
          className="flex h-7 w-7 items-center justify-center rounded-md text-gray-500 transition-colors hover:bg-gray-200 hover:text-gray-700"
          title="New chat"
        >
          <Plus size={16} />
        </button>
      </div>

      {/* Session list */}
      <div className="flex-1 overflow-y-auto">
        {sessions.length === 0 && (
          <div className="px-4 py-8 text-center text-xs text-gray-400">
            No chats yet
          </div>
        )}
        {sessions.map((session) => {
          const isActive = session.id === activeSessionId;
          const dateStr = session.updated_at
            ? format(new Date(session.updated_at), "MMM d")
            : "";

          return (
            <div
              key={session.id}
              onClick={() => session.id != null && onSelect(session.id)}
              className={clsx(
                "group flex cursor-pointer items-center gap-2 px-4 py-3 transition-colors",
                isActive
                  ? "bg-gray-200"
                  : "hover:bg-gray-100"
              )}
            >
              <div className="min-w-0 flex-1">
                <div className="truncate text-sm font-medium text-gray-800">
                  {session.title}
                </div>
                <div className="text-[11px] text-gray-400">{dateStr}</div>
              </div>
              <button
                type="button"
                onClick={(e) => {
                  e.stopPropagation();
                  if (session.id != null) onDelete(session.id);
                }}
                className="hidden h-6 w-6 shrink-0 items-center justify-center rounded text-gray-400 transition-colors hover:bg-gray-300 hover:text-gray-600 group-hover:flex"
                title="Delete chat"
              >
                <X size={14} />
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
}
