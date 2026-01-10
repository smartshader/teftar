import { useState } from "react";
import { useNavigate } from "react-router";
import type { Route } from "./+types/signin";
import { config } from "~/lib/config";
import { useAuthStore } from "~/lib/stores/auth";
import { AuthLayout } from "~/components/layouts/auth-layout";
import { AuthCard } from "~/components/ui/auth-card";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Sign In - Teftar" },
    { name: "description", content: "Sign in to your Teftar account" },
  ];
}

export default function SignIn() {
  const navigate = useNavigate();
  const setAuth = useAuthStore((state) => state.setAuth);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      const response = await fetch(`${config.apiUrl}/auth/signin`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      if (!response.ok) {
        const data = await response.json();
        throw new Error(data.error || "Sign in failed");
      }

      const data = await response.json();

      setAuth(data.access_token, data.refresh_token, data.user);

      navigate("/dashboard");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Something went wrong");
    } finally {
      setLoading(false);
    }
  };

  return (
    <AuthLayout title="Sign in to your account">
      <AuthCard
        title="Sign In"
        description="Enter your credentials to access your account"
      >
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              placeholder="you@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              autoComplete="email"
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              type="password"
              placeholder="••••••••"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              autoComplete="current-password"
            />
          </div>
          {error && <div className="text-sm text-destructive">{error}</div>}
          <Button type="submit" className="w-full" disabled={loading}>
            {loading ? "Signing in..." : "Sign In"}
          </Button>
        </form>
        <div className="mt-4 text-center text-sm text-muted-foreground">
          Don't have an account?{" "}
          <a href="/signup" className="text-foreground hover:underline">
            Sign up
          </a>
        </div>
      </AuthCard>
    </AuthLayout>
  );
}
