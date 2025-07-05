import { Button } from '@/components/ui/button'

export function Login() {
  const signIn = (provider?: string) => {
    const url = provider ? `/api/auth/signin/${provider}` : '/api/auth/signin/email'
    window.location.href = url
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen space-y-4">
      <h1 className="text-2xl font-semibold">Welcome to Swoop</h1>
      <Button onClick={() => signIn('email')}>Sign in with Email</Button>
      <Button onClick={() => signIn('github')}>Sign in with GitHub</Button>
      <Button onClick={() => signIn('google')}>Sign in with Google</Button>
    </div>
  )
} 