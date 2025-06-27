/*
 * BetterAuth – Edge-friendly authentication layer
 * Centralises provider configuration and session helpers so all
 * edge/serverless handlers can cheaply fetch the user object.
 */
import { betterAuth } from 'better-auth'
import { prismaAdapter } from 'better-auth/adapters/prisma'
import { prisma } from './prisma'
import { GitHubProvider, GoogleProvider, EmailProvider } from 'better-auth/providers'
import type { EdgeRequest } from '../types'

// 7-day cookie TTL in seconds
const WEEK = 60 * 60 * 24 * 7

export const auth = betterAuth({
  database: prismaAdapter(prisma, { provider: process.env.PRISMA_DB_PROVIDER || 'mysql' }),
  providers: [
    EmailProvider({
      server: process.env.EMAIL_SERVER!,
      from: process.env.EMAIL_FROM!,
      // Magic-link validity (15 min)
      maxAge: 60 * 15,
    }),
    GitHubProvider({
      clientId: process.env.GITHUB_CLIENT_ID!,
      clientSecret: process.env.GITHUB_CLIENT_SECRET!,
    }),
    GoogleProvider({
      clientId: process.env.GOOGLE_CLIENT_ID!,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET!,
    }),
  ],
  cookies: {
    sessionToken: {
      name: '__Secure-swoop-session',
      options: {
        httpOnly: true,
        sameSite: 'lax',
        path: '/',
        secure: true,
        maxAge: WEEK,
      },
    },
  },
  session: {
    strategy: 'jwt',
    maxAge: WEEK,
  },
  secret: process.env.BETTERAUTH_SECRET!,
})

// --- Typed session helper ---------------------------------------------------

export type Session = Awaited<ReturnType<typeof auth.getSession>>

export async function getSession(req: Request): Promise<Session | null> {
  return auth.getSession(req)
}

// requireUser helper for edge/serverless handlers
export async function requireUser(req: EdgeRequest): Promise<Session['user']> {
  const session = await getSession(req)
  if (!session || !session.user) {
    return Response.json({ error: 'Unauthorised' }, { status: 401 }) as never
  }
  return session.user
} 