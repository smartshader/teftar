import { useEffect } from "react";
import { useNavigate } from "react-router";
import type { Route } from "./+types/dashboard";
import { DashboardLayout } from "~/components/layouts/dashboard-layout";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Construction } from "lucide-react";

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

  return (
    <DashboardLayout>
      <div className="flex items-center justify-center min-h-[calc(100vh-200px)]">
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
      </div>
    </DashboardLayout>
  );
}
