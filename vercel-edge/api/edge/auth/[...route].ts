import { auth } from '../../../lib/auth'

export const runtime = 'edge'

export default function handler(request: Request) {
  // Delegate to BetterAuth which internally handles sign-in, callback, session, etc.
  return auth.handleRequest(request)
} 