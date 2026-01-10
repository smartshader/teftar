import { useState } from "react";
import { useNavigate } from "react-router";
import type { Route } from "./+types/signup";
import { config } from "~/lib/config";
import { AuthLayout } from "~/components/layouts/auth-layout";
import { AuthCard } from "~/components/ui/auth-card";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Sign Up - Teftar" },
    { name: "description", content: "Create your Teftar account" },
  ];
}

export default function SignUp() {
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      const response = await fetch(`${config.apiUrl}/auth/signup`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      // Handle 202 Accepted (confirmation required)
      if (response.status === 202) {
        navigate("/confirm-email");
        return;
      }

      if (!response.ok) {
        const data = await response.json();
        const errorMessage = data.error || "Sign up failed";

        // Fallback: Check if email confirmation is required
        if (
          errorMessage.includes("check your email") ||
          errorMessage.includes("confirm your account")
        ) {
          navigate("/confirm-email");
          return;
        }

        throw new Error(errorMessage);
      }

      const data = await response.json();

      // Store tokens in localStorage
      localStorage.setItem("access_token", data.access_token);
      localStorage.setItem("refresh_token", data.refresh_token);
      localStorage.setItem("user", JSON.stringify(data.user));

      // Redirect to dashboard (will create later)
      navigate("/");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Something went wrong");
    } finally {
      setLoading(false);
    }
  };

  return (
    <AuthLayout title="Create your account to get started">
      <AuthCard
        title="Sign Up"
        description="Enter your email and password to create an account"
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
              minLength={6}
              autoComplete="new-password"
            />
            <p className="text-xs text-muted-foreground">
              Must be at least 6 characters
            </p>
          </div>
          {error && <div className="text-sm text-destructive">{error}</div>}
          <Button type="submit" className="w-full" disabled={loading}>
            {loading ? "Creating account..." : "Create Account"}
          </Button>
        </form>
        <div className="mt-4 text-center text-sm text-muted-foreground">
          Already have an account?{" "}
          <a href="/signin" className="text-foreground hover:underline">
            Sign in
          </a>
        </div>
      </AuthCard>
    </AuthLayout>
  );
}
