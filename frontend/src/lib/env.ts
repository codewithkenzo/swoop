/// <reference types="vite/client" />

// Environment helper for frontend
// Centralizes access to Vite environment variables with sensible fallbacks.
// Keeping this file under 50 LOC to retain modularity.

// Base URL for backend API requests. Defaults to `/api` to support relative proxying during production builds.
export const API_BASE_URL: string =
  (import.meta.env.VITE_API_BASE_URL as string | undefined) || '/api';

// Currently selected LLM provider for UI hints / feature-gating.
export const LLM_PROVIDER: string | undefined =
  import.meta.env.VITE_LLM_PROVIDER as string | undefined;

// Optional analytics DSN (e.g. Sentry, PostHog). Undefined if not set.
export const ANALYTICS_DSN: string | undefined =
  import.meta.env.VITE_ANALYTICS_DSN as string | undefined; 