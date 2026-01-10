import { useEffect } from "react";
import { useNavigate } from "react-router";
import type { Route } from "./+types/dashboard";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Button } from "~/components/ui/button";
import { FileText, Construction } from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Dashboard - Teftar" },
    { name: "description", content: "Your Teftar dashboard" },
  ];
}

export default function Dashboard() {
  const navigate = useNavigate();

  useEffect(() => {
    const accessToken = localStorage.getItem("access_token");
    if (!accessToken) {
      navigate("/signin");
    }
  }, [navigate]);

  const handleSignOut = () => {
    localStorage.removeItem("access_token");
    localStorage.removeItem("refresh_token");
    navigate("/");
  };

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <a
            href="/dashboard"
            className="flex items-center space-x-2 hover:opacity-80 transition-opacity"
          >
            <FileText className="h-6 w-6" />
            <span className="text-xl font-semibold">Teftar</span>
          </a>
          <Button variant="outline" onClick={handleSignOut}>
            Sign Out
          </Button>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8 flex items-center justify-center min-h-[calc(100vh-80px)]">
        <Card className="w-full max-w-md border-amber-200 bg-amber-50/50 dark:bg-amber-950/20 dark:border-amber-900">
          <CardHeader className="text-center">
            <div className="flex justify-center mb-4">
              <div className="rounded-full bg-amber-100 dark:bg-amber-900/50 p-4">
                <Construction className="h-12 w-12 text-amber-600 dark:text-amber-400" />
              </div>
            </div>
            <CardTitle className="text-2xl">Site Under Construction</CardTitle>
          </CardHeader>
          <CardContent className="text-center space-y-4">
            <p className="text-sm text-muted-foreground">
              We're working hard to bring you an amazing accounting experience.
              Check back soon!
            </p>
          </CardContent>
        </Card>
      </main>
    </div>
  );
}
