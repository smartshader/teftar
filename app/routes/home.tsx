import type { Route } from "./+types/home";
import { Button } from "~/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Separator } from "~/components/ui/separator";
import {
  FileText,
  Receipt,
  TrendingUp,
  Users,
  Shield,
  Zap,
} from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Teftar - Accounting & Bookkeeping Software" },
    {
      name: "description",
      content:
        "Simple, complete accounting software for small businesses. Track invoices, expenses, and clients with ease.",
    },
  ];
}

export default function Home() {
  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <FileText className="h-6 w-6" />
            <span className="text-xl font-semibold">Teftar</span>
          </div>
          <nav className="hidden md:flex items-center space-x-6">
            <a
              href="#features"
              className="text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              Features
            </a>
            <Button size="sm">Get Started</Button>
          </nav>
        </div>
      </header>

      {/* Hero Section */}
      <section className="container mx-auto px-4 py-24 md:py-32">
        <div className="max-w-3xl mx-auto text-center space-y-8">
          <div className="space-y-4">
            <h1 className="text-4xl md:text-6xl font-bold tracking-tight">
              Simple Accounting for Small Business
            </h1>
            <p className="text-xl text-muted-foreground">
              Complete and easy-to-use bookkeeping software. Track invoices,
              expenses, and clients without the complexity.
            </p>
          </div>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button size="lg" className="text-base">
              Get Started
            </Button>
            <Button size="lg" variant="outline" className="text-base">
              View Demo
            </Button>
          </div>
          <p className="text-sm text-muted-foreground">
            No credit card required • Get started in minutes
          </p>
        </div>
      </section>

      <Separator className="container mx-auto" />

      {/* Features Section */}
      <section id="features" className="container mx-auto px-4 py-24">
        <div className="text-center space-y-4 mb-16">
          <h2 className="text-3xl md:text-4xl font-bold">
            Everything You Need
          </h2>
          <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
            All the essential features to manage your business finances, nothing
            more, nothing less.
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card>
            <CardHeader>
              <FileText className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Invoice Management</CardTitle>
              <CardDescription>
                Create, send, and track professional invoices. Get paid faster
                with automated reminders.
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <Receipt className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Expense Tracking</CardTitle>
              <CardDescription>
                Record and categorize business expenses. Keep track of receipts
                and stay organized.
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <Users className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Client Management</CardTitle>
              <CardDescription>
                Manage client information, track payment history, and maintain
                relationships.
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <TrendingUp className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Reports & Analytics</CardTitle>
              <CardDescription>
                Understand your business with clear financial reports and
                insights.
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <Shield className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Secure & Private</CardTitle>
              <CardDescription>
                Your financial data is encrypted and protected. We never share
                your information.
              </CardDescription>
            </CardHeader>
          </Card>

          <Card>
            <CardHeader>
              <Zap className="h-10 w-10 mb-2 text-primary" />
              <CardTitle>Fast & Simple</CardTitle>
              <CardDescription>
                No learning curve. Get started in minutes with our intuitive
                interface.
              </CardDescription>
            </CardHeader>
          </Card>
        </div>
      </section>

      {/* CTA Section */}
      <section className="container mx-auto px-4 py-24">
        <div className="max-w-3xl mx-auto text-center space-y-8">
          <h2 className="text-3xl md:text-5xl font-bold">
            Ready to Simplify Your Accounting?
          </h2>
          <p className="text-xl text-muted-foreground">
            Join thousands of small businesses managing their finances with
            Teftar.
          </p>
          <Button size="lg" className="text-base">
            Get Started Today
          </Button>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t">
        <div className="container mx-auto px-4 py-12">
          <div className="flex flex-col items-center space-y-4">
            <div className="flex items-center space-x-2">
              <FileText className="h-5 w-5" />
              <span className="font-semibold">Teftar</span>
            </div>
            <p className="text-sm text-muted-foreground text-center">
              Accounting software for small businesses.
            </p>
            <div className="text-sm text-muted-foreground">
              © 2026 Teftar. All rights reserved.
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}
