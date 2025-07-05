import React from 'react'
import { Link, useLocation } from 'react-router-dom'
import { 
  Home, 
  FileText, 
  Upload, 
  Globe, 
  Search, 
  Settings,
  Brain,
  Menu,
  X,
  Activity
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

interface LayoutProps {
  children: React.ReactNode
}

const navigation = [
  { name: 'Dashboard', href: '/', icon: Home },
  { name: 'Documents', href: '/documents', icon: FileText },
  { name: 'Upload', href: '/upload', icon: Upload },
  { name: 'Web Crawl', href: '/crawl', icon: Globe },
  { name: 'AI Search', href: '/search', icon: Search },
  { name: 'Operations', href: '/monitoring', icon: Activity },
  { name: 'Settings', href: '/settings', icon: Settings },
]

export function Layout({ children }: LayoutProps) {
  const location = useLocation()
  const [sidebarOpen, setSidebarOpen] = React.useState(false)

  return (
    <div className="flex h-screen bg-background">
      {/* Mobile sidebar overlay */}
      {sidebarOpen && (
        <div 
          className="fixed inset-0 z-40 bg-black bg-opacity-25 lg:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <div className={cn(
        "fixed inset-y-0 left-0 z-50 w-64 bg-card border-r transform transition-transform duration-200 ease-in-out lg:translate-x-0 lg:static lg:inset-0",
        sidebarOpen ? "translate-x-0" : "-translate-x-full"
      )}>
        <div className="flex items-center justify-between h-16 px-6 border-b">
          <div className="flex items-center space-x-2">
            <Brain className="h-8 w-8 text-primary" />
            <h1 className="text-xl font-bold">Swoop</h1>
          </div>
          <Button
            variant="ghost"
            size="icon"
            className="lg:hidden"
            onClick={() => setSidebarOpen(false)}
          >
            <X className="h-5 w-5" />
          </Button>
        </div>

        <nav className="mt-8 px-4">
          <ul className="space-y-2">
            {navigation.map((item) => {
              const Icon = item.icon
              const isActive = location.pathname === item.href
              
              return (
                <li key={item.name}>
                  <Link
                    to={item.href}
                    className={cn(
                      "flex items-center px-4 py-2 text-sm font-medium rounded-md transition-colors",
                      isActive
                        ? "bg-primary text-primary-foreground"
                        : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                    )}
                    onClick={() => setSidebarOpen(false)}
                  >
                    <Icon className="mr-3 h-5 w-5" />
                    {item.name}
                  </Link>
                </li>
              )
            })}
          </ul>
        </nav>

        <div className="absolute bottom-4 left-4 right-4">
          <div className="bg-muted/50 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <div className="h-2 w-2 bg-green-500 rounded-full animate-pulse" />
              <span className="text-xs text-muted-foreground">AI Engine Active</span>
            </div>
          </div>
        </div>
      </div>

      {/* Main content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Mobile header */}
        <div className="lg:hidden flex items-center justify-between h-16 px-4 border-b bg-card">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setSidebarOpen(true)}
          >
            <Menu className="h-5 w-5" />
          </Button>
          <div className="flex items-center space-x-2">
            <Brain className="h-6 w-6 text-primary" />
            <span className="font-semibold">Swoop</span>
          </div>
          <div /> {/* Spacer */}
        </div>

        {/* Page content */}
        <main className="flex-1 overflow-y-auto">
          <div className="container mx-auto px-4 py-8 max-w-7xl">
            {children}
          </div>
        </main>
      </div>
    </div>
  )
} 