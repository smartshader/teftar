import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import type { Route } from "./+types/auth-callback";
import { config } from "~/lib/config";
import { AuthLayout } from "~/components/layouts/auth-layout";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Button } from "~/components/ui/button";
import { CheckCircle2, XCircle } from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Email Confirmed - Teftar" },
    { name: "description", content: "Your email has been confirmed" },
  ];
}

export default function AuthCallback() {
  const navigate = useNavigate();
  const [status, setStatus] = useState<"loading" | "success" | "error">(
    "loading",
  );
  const [error, setError] = useState("");

  useEffect(() => {
    const verifyEmail = async () => {
      // Extract parameters from URL - check both query params and hash
      const searchParams = new URLSearchParams(window.location.search);
      const hashParams = new URLSearchParams(window.location.hash.substring(1));

      // Try query params first (standard for token_hash), then hash (for access_token)
      const tokenHash =
        searchParams.get("token_hash") || hashParams.get("token_hash");
      const accessToken = hashParams.get("access_token");
      const refreshToken = hashParams.get("refresh_token");
      const type = searchParams.get("type") || hashParams.get("type");
      const email = searchParams.get("email") || hashParams.get("email");
      const errorParam = searchParams.get("error") || hashParams.get("error");
      const errorDescription =
        searchParams.get("error_description") ||
        hashParams.get("error_description");

      console.log("URL search:", window.location.search);
      console.log("URL hash:", window.location.hash);
      console.log("Extracted:", { tokenHash, accessToken, type, email });

      if (errorParam) {
        setStatus("error");
        setError(errorDescription || errorParam);
        return;
      }

      // Determine which token we have
      const token = tokenHash || accessToken;

      if (!token || !type) {
        setStatus("error");
        setError("Invalid confirmation link. Missing parameters.");
        return;
      }

      // Verify the OTP token with backend
      try {
        const response = await fetch(`${config.apiUrl}/auth/verify-email`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            token: token,
            type_: type,
            email: email || undefined,
          }),
        });

        if (!response.ok) {
          const data = await response.json();
          throw new Error(data.error || "Email verification failed");
        }

        const data = await response.json();

        // Store tokens and user info
        localStorage.setItem("access_token", data.access_token);
        localStorage.setItem("refresh_token", data.refresh_token);
        localStorage.setItem("user", JSON.stringify(data.user));

        setStatus("success");
      } catch (err) {
        setStatus("error");
        setError(
          err instanceof Error ? err.message : "Email verification failed",
        );
      }
    };

    verifyEmail();
  }, []);

  const handleContinue = () => {
    navigate("/dashboard");
  };

  if (status === "loading") {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center p-4">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
          <p className="mt-4 text-muted-foreground">Confirming your email...</p>
        </div>
      </div>
    );
  }

  if (status === "error") {
    return (
      <AuthLayout>
        <Card>
          <CardHeader className="text-center">
            <div className="flex justify-center mb-4">
              <div className="rounded-full bg-destructive/10 p-4">
                <XCircle className="h-12 w-12 text-destructive" />
              </div>
            </div>
            <CardTitle className="text-2xl">Confirmation Failed</CardTitle>
          </CardHeader>
          <CardContent className="text-center space-y-4">
            <p className="text-sm text-muted-foreground">
              {error || "The confirmation link is invalid or has expired."}
            </p>
            <a href="/signup">
              <Button className="w-full">Back to Sign Up</Button>
            </a>
          </CardContent>
        </Card>
      </AuthLayout>
    );
  }

  return (
    <AuthLayout>
      <Card className="border-green-200 bg-green-50/50 dark:bg-green-950/20 dark:border-green-900">
        <CardHeader className="text-center">
          <div className="flex justify-center mb-4">
            <div className="rounded-full bg-green-100 dark:bg-green-900/50 p-4">
              <CheckCircle2 className="h-12 w-12 text-green-600 dark:text-green-400" />
            </div>
          </div>
          <CardTitle className="text-2xl">Email Confirmed!</CardTitle>
        </CardHeader>
        <CardContent className="text-center space-y-4">
          <p className="text-sm text-muted-foreground">
            Your email has been successfully confirmed. You can now start using
            Teftar.
          </p>
          <Button onClick={handleContinue} className="w-full">
            Continue to Dashboard
          </Button>
        </CardContent>
      </Card>
    </AuthLayout>
  );
}
