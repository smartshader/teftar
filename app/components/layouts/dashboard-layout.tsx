import { useNavigate } from "react-router";
import { BrandLogo } from "~/components/brand-logo";
import { Button } from "~/components/ui/button";
import { useAuthStore } from "~/lib/stores/auth";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export function DashboardLayout({ children }: DashboardLayoutProps) {
  const navigate = useNavigate();
  const clearAuth = useAuthStore((state) => state.clearAuth);

  const handleSignOut = () => {
    clearAuth();
    navigate("/");
  };

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div className="flex items-center gap-8">
            <BrandLogo size="md" linkTo="/dashboard" />
            <nav className="flex items-center gap-6">
              <a
                href="/clients"
                className="text-sm text-muted-foreground hover:text-foreground transition-colors"
              >
                Clients
              </a>
            </nav>
          </div>
          <Button variant="outline" onClick={handleSignOut}>
            Sign Out
          </Button>
        </div>
      </header>
      <main className="container mx-auto px-4 py-8">{children}</main>
    </div>
  );
}
