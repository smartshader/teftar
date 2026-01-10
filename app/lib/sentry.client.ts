import * as Sentry from "@sentry/react";

export function initSentry() {
  const dsn = import.meta.env.VITE_SENTRY_DSN;
  const environment =
    import.meta.env.VITE_SENTRY_ENVIRONMENT || import.meta.env.MODE;
  const apiUrl = import.meta.env.VITE_API_URL || "http://localhost:8080";

  if (!dsn) {
    console.warn("Sentry DSN not configured. Error tracking disabled.");
    return;
  }

  Sentry.init({
    dsn,
    environment,
    sendDefaultPii: true,

    integrations: [
      Sentry.browserTracingIntegration(),
      Sentry.replayIntegration({
        maskAllText: true,
        blockAllMedia: true,
      }),
    ],

    tracesSampleRate: environment === "production" ? 0.1 : 1.0,

    tracePropagationTargets: [
      "localhost",
      /^https:\/\/[^/]+\.fly\.dev/,
      new RegExp(`^${apiUrl.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}`),
    ],

    replaysSessionSampleRate: 0.01,
    replaysOnErrorSampleRate: 1.0,

    enableLogs: true,

    ignoreErrors: [
      "ResizeObserver loop limit exceeded",
      "ResizeObserver loop completed with undelivered notifications",
      "Non-Error promise rejection captured",
      /^cancelAnimationFrame/,
      /^clearTimeout/,
    ],

    enabled: environment !== "development",

    beforeSend(event, hint) {
      if (event.exception) {
        const frames = event.exception.values?.[0]?.stacktrace?.frames;
        if (
          frames?.some(
            (frame) =>
              frame.filename?.includes("extension://") ||
              frame.filename?.includes("chrome-extension://") ||
              frame.filename?.includes("moz-extension://"),
          )
        ) {
          return null;
        }
      }
      return event;
    },
  });
}
