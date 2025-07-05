import './globals.css';
import { RootProvider } from 'fumadocs-ui/provider';
import { Inter, JetBrains_Mono } from 'next/font/google';
import type { ReactNode } from 'react';

const inter = Inter({
  subsets: ['latin'],
  variable: '--font-inter',
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono',
});

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" className={`${inter.variable} ${jetbrainsMono.variable}`}>
      <body>
        <RootProvider
          theme={{
            enabled: true,
            defaultTheme: 'system',
          }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}

export const metadata = {
  title: {
    template: '%s | Swoop Documentation',
    default: 'Swoop Documentation',
  },
  description: 'AI-Powered Document Intelligence Platform - Complete documentation and API reference',
  keywords: ['document processing', 'AI', 'search', 'REST API', 'Rust', 'React'],
  authors: [{ name: 'Swoop Team' }],
  creator: 'Swoop',
  openGraph: {
    title: 'Swoop Documentation',
    description: 'AI-Powered Document Intelligence Platform',
    url: 'https://docs.swoop.dev',
    siteName: 'Swoop Docs',
    images: [
      {
        url: '/og-image.png',
        width: 1200,
        height: 630,
        alt: 'Swoop - AI-Powered Document Intelligence',
      },
    ],
    locale: 'en_US',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Swoop Documentation',
    description: 'AI-Powered Document Intelligence Platform',
    images: ['/og-image.png'],
  },
  icons: {
    icon: '/favicon.ico',
    shortcut: '/favicon-16x16.png',
    apple: '/apple-touch-icon.png',
  },
  manifest: '/site.webmanifest',
};