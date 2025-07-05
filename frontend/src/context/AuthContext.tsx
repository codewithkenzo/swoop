import React, { createContext, useContext, useEffect, useState } from 'react'

interface SessionUser {
  id: string
  email: string
  name?: string
  image?: string
  role?: string
}

interface Session {
  user?: SessionUser
  expires?: string
}

interface AuthContextValue {
  session: Session | null
  loading: boolean
}

const AuthContext = createContext<AuthContextValue>({ session: null, loading: true })

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [session, setSession] = useState<Session | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    async function load() {
      try {
        const res = await fetch('/api/auth/session')
        if (res.ok) {
          const data = await res.json()
          setSession(data.session ?? null)
        }
      } catch {
        // ignore network errors
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [])

  return <AuthContext.Provider value={{ session, loading }}>{children}</AuthContext.Provider>
}

export const useSession = () => useContext(AuthContext) 