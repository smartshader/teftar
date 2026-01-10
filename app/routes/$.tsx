import type { Route } from "./+types/$";

export function loader({ request }: Route.LoaderArgs) {
  const url = new URL(request.url);

  // Silently handle well-known requests from browsers/tools
  if (url.pathname.startsWith("/.well-known/")) {
    return new Response(null, { status: 404 });
  }

  // For other 404s, throw to show error page
  throw new Response("Not Found", { status: 404 });
}

export default function NotFound() {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-2">404</h1>
        <p className="text-muted-foreground mb-4">Page not found</p>
        <a href="/" className="text-primary hover:underline">
          Go home
        </a>
      </div>
    </div>
  );
}
