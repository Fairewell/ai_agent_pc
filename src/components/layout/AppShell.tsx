import { Outlet } from "react-router-dom";
import { NavLink } from "react-router-dom";
import { Search, LayoutDashboard, Settings, Database } from "lucide-react";
import clsx from "clsx";
import { useIndexStore } from "../../stores/indexStore";

const navItems = [
  { to: "/", label: "Search", icon: Search },
  { to: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
  { to: "/settings", label: "Settings", icon: Settings },
];

export function AppShell() {
  const status = useIndexStore((s) => s.status);

  return (
    <div className="flex h-screen">
      {/* Sidebar */}
      <aside className="flex w-60 flex-col bg-gray-900 text-white">
        {/* App title */}
        <div className="px-5 py-5">
          <h1 className="text-lg font-bold tracking-tight">AI Agent PC</h1>
        </div>

        {/* Navigation */}
        <nav className="flex-1 px-3">
          <ul className="space-y-1">
            {navItems.map(({ to, label, icon: Icon }) => (
              <li key={to}>
                <NavLink
                  to={to}
                  end={to === "/"}
                  className={({ isActive }) =>
                    clsx(
                      "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                      isActive
                        ? "bg-gray-700 text-white"
                        : "text-gray-400 hover:bg-gray-800 hover:text-white"
                    )
                  }
                >
                  <Icon size={18} />
                  {label}
                </NavLink>
              </li>
            ))}
          </ul>
        </nav>

        {/* Index status indicator */}
        <div className="border-t border-gray-800 px-5 py-4">
          <div className="flex items-center gap-2 text-xs text-gray-400">
            <Database size={14} />
            <span>
              {status.indexed_files.toLocaleString()} files indexed
            </span>
          </div>
          {status.is_scanning && (
            <div className="mt-1 text-xs text-blue-400">Scanning...</div>
          )}
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto bg-gray-50">
        <Outlet />
      </main>
    </div>
  );
}
