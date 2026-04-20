import { NavLink, Outlet } from "react-router";

const navItems = [
  { to: "/", label: "Schedule" },
  { to: "/promotions", label: "Promotions" },
  { to: "/championships", label: "Championships" },
  { to: "/settings", label: "Settings" },
];

function Layout() {
  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white border-b border-gray-200">
        <div className="mx-auto max-w-7xl px-4">
          <div className="flex h-14 items-center justify-between">
            <div className="flex items-center gap-8">
              <span className="text-xl font-bold text-gray-900">Suplex</span>
              <div className="flex gap-1">
                {navItems.map((item) => (
                  <NavLink
                    key={item.to}
                    to={item.to}
                    end={item.to === "/"}
                    className={({ isActive }) =>
                      `rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                        isActive
                          ? "bg-gray-100 text-gray-900"
                          : "text-gray-500 hover:bg-gray-50 hover:text-gray-700"
                      }`
                    }
                  >
                    {item.label}
                  </NavLink>
                ))}
              </div>
            </div>
          </div>
        </div>
      </nav>
      <main className="mx-auto max-w-7xl px-4 py-6">
        <Outlet />
      </main>
    </div>
  );
}

export default Layout;
