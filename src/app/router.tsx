import { Routes, Route } from "react-router-dom";
import { AppShell } from "../components/layout/AppShell";
import { SearchPage } from "./routes/SearchPage";
import { DashboardPage } from "./routes/DashboardPage";
import { SettingsPage } from "./routes/SettingsPage";
import { ChatPage } from "./routes/ChatPage";

export function AppRouter() {
  return (
    <Routes>
      <Route element={<AppShell />}>
        <Route path="/" element={<SearchPage />} />
        <Route path="/chat" element={<ChatPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}
