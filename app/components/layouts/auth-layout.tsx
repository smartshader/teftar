import { BrandLogo } from "~/components/brand-logo";

interface AuthLayoutProps {
  children: React.ReactNode;
  title?: string;
  subtitle?: string;
}

export function AuthLayout({ children, title, subtitle }: AuthLayoutProps) {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="w-full max-w-md space-y-8">
        <div className="flex flex-col items-center space-y-2">
          <BrandLogo size="lg" linkTo="/" />
          {(title || subtitle) && (
            <div className="text-center space-y-1">
              {title && <h1 className="text-xl font-semibold">{title}</h1>}
              {subtitle && (
                <p className="text-muted-foreground text-sm">{subtitle}</p>
              )}
            </div>
          )}
        </div>
        {children}
      </div>
    </div>
  );
}
