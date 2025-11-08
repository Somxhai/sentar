// app/(creator)/layout.tsx
import Link from "next/link"
import { ReactNode } from "react"
import { UserMenu } from "@/components/auth/UserMenu"

export default function CreatorLayout({ children }: { children: ReactNode }) {
  return (
    <div className="min-h-dvh bg-background text-foreground">
      <header className="border-b">
        <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-4">
          <Link href="/workspaces" className="font-semibold">Sentar • Creator</Link>
          <nav className="flex items-center gap-4 text-sm">
            <Link href="/workspaces" className="hover:underline">Workspaces</Link>
            <Link href="/workspaces/new" className="hover:underline">New workspace</Link>
            <UserMenu />
          </nav>
        </div>
      </header>
      <main className="mx-auto max-w-6xl px-4 py-6">{children}</main>
    </div>
  )
}
