import { useNavigate, useLocation } from "react-router";
import { BrandLogo } from "~/components/brand-logo";
import { Button } from "~/components/ui/button";
import { useAuthStore } from "~/lib/stores/auth";
import { LayoutDashboard, Users, LogOut } from "lucide-react";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export function DashboardLayout({ children }: DashboardLayoutProps) {
  const navigate = useNavigate();
  const location = useLocation();
  const clearAuth = useAuthStore((state) => state.clearAuth);

  const handleSignOut = () => {
    clearAuth();
    navigate("/");
  };

  const isActive = (path: string) => location.pathname === path;

  return (
    <div className="flex min-h-screen bg-background">
      {/* Sidebar */}
      <aside className="w-64 border-r bg-muted/10 flex flex-col">
        <div className="p-6 border-b">
          <BrandLogo size="md" linkTo="/dashboard" />
        </div>

        <nav className="flex-1 p-4 space-y-1">
          <a
            href="/dashboard"
            className={`flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors ${
              isActive("/dashboard")
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted hover:text-foreground"
            }`}
          >
            <LayoutDashboard className="h-4 w-4" />
            Dashboard
          </a>
          <a
            href="/clients"
            className={`flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors ${
              isActive("/clients")
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted hover:text-foreground"
            }`}
          >
            <Users className="h-4 w-4" />
            Clients
          </a>
        </nav>

        <div className="p-4 border-t">
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={handleSignOut}
          >
            <LogOut className="h-4 w-4 mr-3" />
            Sign Out
          </Button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-auto">
        <div className="container mx-auto px-8 py-8">{children}</div>
      </main>
    </div>
  );
}
