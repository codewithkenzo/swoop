/// <reference types="vite/client" />

// Environment helper for frontend
// Centralizes access to Vite environment variables with sensible fallbacks.
// Keeping this file under 50 LOC to retain modularity.

// Primary backend base URL – new name VITE_BACKEND_URL, fallback to legacy VITE_API_BASE_URL
const rawBase: string | undefined = (
  import.meta.env.VITE_BACKEND_URL as string | undefined) ??
  (import.meta.env.VITE_API_BASE_URL as string | undefined);

// Normalise: if a host is provided without the `/api` suffix we append it.
export const API_BASE_URL: string = (() => {
  if (!rawBase) {
    // When no env var is set we assume the backend is served from the same origin
    // and is mounted under /api (dev-server proxy or prod reverse-proxy).
    // Using location.origin ensures absolute URL for EventSource to avoid CORS preflight.
    if (typeof window !== 'undefined' && window.location) {
      return `${window.location.origin}/api`;
    }
    return '/api';
  }
  return rawBase.endsWith('/api') ? rawBase : `${rawBase.replace(/\/*$/, '')}/api`;
})();

// Currently selected LLM provider for UI hints / feature-gating.
export const LLM_PROVIDER: string | undefined =
  import.meta.env.VITE_LLM_PROVIDER as string | undefined;

// Optional analytics DSN (e.g. Sentry, PostHog). Undefined if not set.
export const ANALYTICS_DSN: string | undefined =
  import.meta.env.VITE_ANALYTICS_DSN as string | undefined; 