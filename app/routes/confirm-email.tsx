import type { Route } from "./+types/confirm-email";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { FileText, Mail } from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Confirm Your Email - Teftar" },
    {
      name: "description",
      content: "Please check your email to confirm your account",
    },
  ];
}

export default function ConfirmEmail() {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="w-full max-w-md space-y-8">
        <div className="flex flex-col items-center space-y-2">
          <div className="flex items-center space-x-2">
            <FileText className="h-8 w-8" />
            <span className="text-2xl font-semibold">Teftar</span>
          </div>
        </div>

        <Card className="border-blue-200 bg-blue-50/50 dark:bg-blue-950/20 dark:border-blue-900">
          <CardHeader className="text-center">
            <div className="flex justify-center mb-4">
              <div className="rounded-full bg-blue-100 dark:bg-blue-900/50 p-4">
                <Mail className="h-12 w-12 text-blue-600 dark:text-blue-400" />
              </div>
            </div>
            <CardTitle className="text-2xl">Check Your Email</CardTitle>
            <CardDescription className="text-base pt-2">
              We've sent a confirmation link to your email address.
            </CardDescription>
          </CardHeader>
          <CardContent className="text-center space-y-4">
            <p className="text-sm text-muted-foreground">
              Please click the link in the email to confirm your account and
              complete the signup process.
            </p>
            <p className="text-xs text-muted-foreground">
              Didn't receive the email? Check your spam folder.
            </p>
          </CardContent>
        </Card>

        <div className="text-center text-sm text-muted-foreground">
          Already confirmed?{" "}
          <a href="/signin" className="text-foreground hover:underline">
            Sign in
          </a>
        </div>
      </div>
    </div>
  );
}
