# AI Agent PC

Desktop application for local document indexing, full-text search, and AI-powered document analysis. Built with Tauri v2, React, and Rust.

## Features

- **Document Indexing** — scan directories, extract text, and build a searchable index with automatic change detection (BLAKE3 hashing)
- **Full-Text Search** — fast local search powered by Tantivy (Rust-native Lucene alternative)
- **AI Chat** — conversational interface with streaming responses, chat sessions, and file context
- **RAG Pipeline** — search-augmented generation: finds relevant documents, extracts context, and sends to LLM
- **AI Agent** — multi-step tool-calling agent with built-in tools: `search_documents`, `read_file`, `list_files`, `transform_document`, `calculate`
- **Provider System** — connect any OpenAI-compatible API: OpenAI, Ollama (local), OpenRouter, LM Studio, or custom endpoints
- **Streaming** — real-time SSE streaming of LLM responses via Tauri events

## Tech Stack

### Frontend
| Technology | Purpose |
|---|---|
| React 19 | UI framework |
| TypeScript 5.8 | Type safety |
| Tailwind CSS 4 | Styling |
| Zustand 5 | State management |
| TanStack Query 5 | Async data fetching |
| React Router 7 | Navigation |
| React Virtuoso | Virtualized lists |
| Lucide React | Icons |

### Backend (Rust)
| Crate | Purpose |
|---|---|
| Tauri 2 | Desktop runtime & IPC |
| Tantivy 0.22 | Full-text search engine |
| rusqlite | SQLite for metadata storage |
| reqwest | HTTP client for LLM APIs (streaming) |
| walkdir | Recursive directory scanning |
| notify | Filesystem watcher |
| blake3 | Fast content hashing |
| tokio | Async runtime |
| tracing | Structured logging |

## Project Structure

```
ai_agent_pc/
├── src/                        # Frontend (React/TS)
│   ├── app/
│   │   ├── router.tsx
│   │   └── routes/
│   │       ├── ChatPage.tsx        # AI chat interface
│   │       ├── DashboardPage.tsx   # Index status overview
│   │       ├── SearchPage.tsx      # Full-text search + AI prompt
│   │       └── SettingsPage.tsx    # Provider management
│   ├── components/
│   │   ├── chat/               # Chat UI components
│   │   ├── layout/             # AppShell, sidebar
│   │   ├── search/             # SearchBar, ResultCard, PromptBar
│   │   └── settings/           # Provider forms & presets
│   ├── hooks/                  # useChat, useSearch, useProviders, ...
│   ├── services/               # Tauri command wrappers
│   ├── stores/                 # Zustand stores
│   └── types/                  # TypeScript interfaces
├── src-tauri/                  # Backend (Rust)
│   ├── src/
│   │   ├── ai/
│   │   │   ├── agent.rs        # Multi-step agent loop
│   │   │   ├── chat.rs         # Chat completion calls
│   │   │   ├── provider.rs     # Provider CRUD & config
│   │   │   ├── rag.rs          # RAG pipeline
│   │   │   ├── streaming.rs    # SSE stream parsing
│   │   │   └── tools.rs        # Agent tool definitions
│   │   ├── commands/           # Tauri IPC commands
│   │   ├── db/                 # SQLite schema, queries, models
│   │   ├── extractors/         # Text extraction (plaintext)
│   │   ├── indexer/            # Pipeline + directory scanner
│   │   ├── search/             # Tantivy index wrapper
│   │   ├── state.rs            # App state (DB, index handles)
│   │   ├── utils/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## Prerequisites

- **Node.js** >= 18
- **Rust** >= 1.77 (with `cargo`)
- **System libraries** (platform-specific):

### Windows
No additional system libraries required.

### macOS
```bash
xcode-select --install
```

### Linux (Debian/Ubuntu)
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev
```

## Installation

```bash
# Clone the repository
git clone https://github.com/Fairewell/ai_agent_pc.git
cd ai_agent_pc

# Install frontend dependencies
npm install

# Run in development mode (starts both Vite dev server and Tauri)
npm run tauri dev

# Build for production
npm run tauri build
```

The built installer/executable will be in `src-tauri/target/release/bundle/`.

## Configuration

### Adding an AI Provider

1. Open the app and go to **Settings**
2. Click **Add Provider**
3. Choose a preset (OpenAI, Ollama, OpenRouter) or configure manually:
   - **Name** — display name
   - **Base URL** — API endpoint (e.g., `https://api.openai.com/v1` or `http://localhost:11434/v1`)
   - **API Key** — your API key (not needed for local Ollama)
   - **Model** — model identifier (e.g., `gpt-4o`, `llama3.1`)
4. Click **Test Connection** to verify, then **Save**

### Indexing Documents

1. Go to **Dashboard**
2. Click **Add Directory** and select a folder
3. The indexer will scan files, extract text, and build the search index
4. Progress and stats are shown on the Dashboard

## Usage

- **Search** — type a query in the Search page to find documents by content
- **AI Prompt** — use the prompt bar on the Search page to ask questions about your documents (RAG)
- **Chat** — open the Chat page for multi-turn conversations with AI, with document context
- **Dashboard** — monitor indexing status and statistics

## Development

```bash
# Frontend only (no Tauri)
npm run dev

# Type check
npx tsc --noEmit

# Rust check
cd src-tauri && cargo check

# Run with Tauri dev mode (hot reload)
npm run tauri dev
```

## License

MIT
