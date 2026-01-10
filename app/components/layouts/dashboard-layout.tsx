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
          <BrandLogo size="md" linkTo="/dashboard" />
          <Button variant="outline" onClick={handleSignOut}>
            Sign Out
          </Button>
        </div>
      </header>
      <main className="container mx-auto px-4 py-8">{children}</main>
    </div>
  );
}
