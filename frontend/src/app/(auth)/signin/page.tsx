// app/(auth)/signin/page.tsx
"use client"

import { useState } from "react"
import { useRouter, useSearchParams } from "next/navigation"
import { signIn, useSession } from "@/lib/auth-client"

export default function SignInPage() {
  const router = useRouter()
  const sp = useSearchParams()
  const next = sp.get("next") ?? "/workspaces"
  const { data: session } = useSession()
  const [email, setEmail] = useState("")
  const [password, setPassword] = useState("")
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  if (session?.user) router.replace(next)

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError(null)
    try {
      await signIn.email({ email, password })
      router.replace(next)
    } catch (err: any) {
      setError(err?.message ?? "Sign in failed")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="mx-auto mt-16 w-full max-w-sm">
      <h1 className="mb-4 text-xl font-semibold">Sign in</h1>
      <form onSubmit={onSubmit} className="space-y-3">
        <label className="block">
          <div className="mb-1 text-sm text-muted-foreground">Email</div>
          <input
            type="email"
            className="w-full rounded-md border px-3 py-2 text-sm outline-none focus:ring-2"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="you@example.com"
            required
          />
        </label>

        <label className="block">
          <div className="mb-1 text-sm text-muted-foreground">Password</div>
          <input
            type="password"
            className="w-full rounded-md border px-3 py-2 text-sm outline-none focus:ring-2"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="••••••••"
            required
          />
        </label>

        {error && <p className="text-sm text-destructive">{error}</p>}

        <button
          type="submit"
          disabled={loading}
          className="inline-flex w-full items-center justify-center rounded-md border px-3 py-2 text-sm hover:bg-accent disabled:opacity-60"
        >
          {loading ? "Signing in..." : "Sign in"}
        </button>
      </form>

      <p className="mt-3 text-sm text-muted-foreground">
        ยังไม่มีบัญชี?{" "}
        <a className="underline" href={`/signup?next=${encodeURIComponent(next)}`}>
          ไปสมัครก่อน
        </a>
      </p>
    </div>
  )
}
