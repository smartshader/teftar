// Environment configuration
// Access environment variables in a type-safe way

export const config = {
  apiUrl: import.meta.env.VITE_API_URL || 'http://localhost:8080',
  siteUrl: import.meta.env.VITE_SITE_URL || 'http://localhost:5173',
} as const;

// Type declarations for Vite env
declare global {
  interface ImportMetaEnv {
    readonly VITE_API_URL: string;
    readonly VITE_SITE_URL: string;
  }

  interface ImportMeta {
    readonly env: ImportMetaEnv;
  }
}
