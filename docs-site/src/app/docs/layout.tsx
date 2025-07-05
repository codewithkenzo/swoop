import { DocsLayout } from 'fumadocs-ui/layout';
import type { ReactNode } from 'react';
import { source } from '@/lib/source';
import { RootToggle } from 'fumadocs-ui/components/layout/root-toggle';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsLayout
      tree={source.pageTree}
      nav={{
        title: (
          <div className="flex items-center gap-2">
            <div className="h-6 w-6 bg-primary rounded flex items-center justify-center">
              <span className="text-primary-foreground font-bold text-sm">S</span>
            </div>
            <span className="font-bold">Swoop Docs</span>
          </div>
        ),
        transparentMode: 'top',
      }}
      sidebar={{
        enabled: true,
        collapsible: true,
        banner: (
          <RootToggle
            options={[
              {
                title: 'Documentation',
                description: 'Complete guides and API reference',
                url: '/docs',
                icon: (
                  <div className="h-6 w-6 bg-primary rounded flex items-center justify-center">
                    <span className="text-primary-foreground font-bold text-sm">D</span>
                  </div>
                ),
              },
              {
                title: 'API Reference',
                description: 'Interactive API documentation',
                url: '/docs/api',
                icon: (
                  <div className="h-6 w-6 bg-green-500 rounded flex items-center justify-center">
                    <span className="text-white font-bold text-sm">A</span>
                  </div>
                ),
              },
            ]}
          />
        ),
        footer: (
          <div className="text-xs text-muted-foreground p-4 border-t">
            <div className="mb-2">
              <strong>Swoop v0.2.0</strong>
            </div>
            <div className="space-y-1">
              <div>🚀 Production Ready</div>
              <div>⚡ Sub-second Processing</div>
              <div>🧠 200+ AI Models</div>
            </div>
          </div>
        ),
      }}
    >
      {children}
    </DocsLayout>
  );
}