import { Suspense } from 'react'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { ThemeProvider } from '@/components/theme-provider'
import { useSession } from './context/AuthContext'
import { Login } from '@/pages/Login'

import { Layout } from '@/components/layout/Layout'
import { Dashboard } from '@/pages/Dashboard'
import { Documents } from '@/pages/Documents'
import { DocumentDetail } from '@/pages/DocumentDetail'
import { Upload } from '@/pages/Upload'
import { Crawl } from '@/pages/Crawl'
import { Search } from '@/pages/Search'
import { Settings } from '@/pages/Settings'
import { RealtimeMonitoring } from '@/pages/RealtimeMonitoring'

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      retry: 2,
    },
  },
})

function App() {
  const { session, loading } = useSession()
  if (loading) {
    return <div className="flex items-center justify-center h-screen">Loading...</div>
  }

  if (!session?.user) {
    return <Login />
  }

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <Router>
          <div className="min-h-screen bg-background">
            <Layout>
              <Suspense fallback={<div className="flex items-center justify-center h-64">Loading...</div>}>
                <Routes>
                  <Route path="/" element={<Dashboard />} />
                  <Route path="/documents" element={<Documents />} />
                  <Route path="/documents/:id" element={<DocumentDetail />} />
                  <Route path="/upload" element={<Upload />} />
                  <Route path="/crawl" element={<Crawl />} />
                  <Route path="/search" element={<Search />} />
                  <Route path="/monitoring" element={<RealtimeMonitoring />} />
                  <Route path="/settings" element={<Settings />} />
                </Routes>
              </Suspense>
            </Layout>
          </div>
        </Router>
      </ThemeProvider>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

export default App 