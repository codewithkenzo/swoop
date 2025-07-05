import * as React from "react";

interface ThemeContextValue {
  theme: "light" | "dark" | "system";
  setTheme: (theme: "light" | "dark" | "system") => void;
}

const ThemeContext = React.createContext<ThemeContextValue | undefined>(undefined);

const STORAGE_KEY = "swoop-theme";

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setThemeState] = React.useState<"light" | "dark" | "system">(() => {
    if (typeof window === "undefined") return "system";
    return (localStorage.getItem(STORAGE_KEY) as "light" | "dark" | "system") || "system";
  });

  React.useEffect(() => {
    const root = window.document.documentElement;
    if (theme === "dark" || (theme === "system" && window.matchMedia("(prefers-color-scheme: dark)").matches)) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
    localStorage.setItem(STORAGE_KEY, theme);
    // TODO: POST to /api/user/preferences in future
  }, [theme]);

  const value = React.useMemo(() => ({ theme, setTheme: setThemeState }), [theme]);
  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}

export function useTheme() {
  const ctx = React.useContext(ThemeContext);
  if (!ctx) throw new Error("useTheme must be used within ThemeProvider");
  return ctx;
} 