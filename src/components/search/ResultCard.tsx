import { FileText, FileImage, FileCode, File } from "lucide-react";
import { format } from "date-fns";
import type { SearchResult } from "../../types/search";
import { openFile } from "../../services/tauriCommands";

function getFileIcon(extension: string) {
  const imageExts = ["png", "jpg", "jpeg", "gif", "bmp", "svg", "webp"];
  const codeExts = ["ts", "tsx", "js", "jsx", "py", "rs", "go", "c", "cpp", "h", "java", "rb", "css", "html"];
  const docExts = ["pdf", "doc", "docx", "txt", "md", "rtf", "odt"];

  const ext = extension.toLowerCase().replace(".", "");

  if (imageExts.includes(ext)) return FileImage;
  if (codeExts.includes(ext)) return FileCode;
  if (docExts.includes(ext)) return FileText;
  return File;
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

function truncatePath(path: string, maxLen = 60): string {
  if (path.length <= maxLen) return path;
  return "..." + path.slice(path.length - maxLen);
}

interface ResultCardProps {
  result: SearchResult;
}

export function ResultCard({ result }: ResultCardProps) {
  const Icon = getFileIcon(result.extension);

  const handleClick = () => {
    openFile(result.path).catch((err) =>
      console.error("Failed to open file:", err)
    );
  };

  let formattedDate = "";
  try {
    formattedDate = format(new Date(result.modified_at), "MMM d, yyyy");
  } catch {
    formattedDate = result.modified_at;
  }

  return (
    <div
      onClick={handleClick}
      className="cursor-pointer rounded-lg border border-gray-200 bg-white p-4 transition-shadow hover:shadow-md"
    >
      <div className="flex items-start gap-3">
        <Icon size={20} className="mt-0.5 shrink-0 text-gray-500" />
        <div className="min-w-0 flex-1">
          <h3 className="truncate text-sm font-semibold text-gray-900">
            {result.filename}
          </h3>
          <p className="mt-0.5 truncate text-xs text-gray-500">
            {truncatePath(result.path)}
          </p>
          {result.snippet && (
            <p className="mt-2 line-clamp-2 text-sm text-gray-600">
              {result.snippet}
            </p>
          )}
          <div className="mt-2 flex items-center gap-3 text-xs text-gray-400">
            <span>{formatFileSize(result.size_bytes)}</span>
            <span>{formattedDate}</span>
          </div>
        </div>
      </div>
    </div>
  );
}
