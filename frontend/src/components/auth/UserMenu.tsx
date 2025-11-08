// components/auth/UserMenu.tsx
"use client"

import { useSession, signOut } from "@/lib/auth-client"
import Link from "next/link"

export function UserMenu() {
  const { data, isPending } = useSession()

  if (isPending) return <div className="text-sm text-muted-foreground">loading…</div>

  if (!data?.user) {
    return (
      <div className="flex items-center gap-2 text-sm">
        <Link href="/signin" className="hover:underline">Sign in</Link>
        <span className="text-muted-foreground">/</span>
        <Link href="/signup" className="hover:underline">Sign up</Link>
      </div>
    )
  }

  return (
    <div className="flex items-center gap-3 text-sm">
      <span className="truncate max-w-40">{data.user.email ?? data.user.name}</span>
      <button
        onClick={() => signOut()} 
        className="rounded-md border px-2 py-1 hover:bg-accent"
      >
        Sign out
      </button>
    </div>
  )
}
