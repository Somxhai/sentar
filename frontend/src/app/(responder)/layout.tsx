// app/(responder)/layout.tsx
import type { ReactNode } from "react"
import Link from "next/link"

export default function ResponderLayout({ children }: { children: ReactNode }) {
  return (
    <div className="min-h-dvh bg-background text-foreground">
      <header className="border-b">
        <div className="mx-auto flex h-12 max-w-4xl items-center justify-between px-4">
          <Link href="/" className="text-sm font-medium">Sentar</Link>
          <div className="text-xs text-muted-foreground">Registration</div>
        </div>
      </header>
      <main className="mx-auto max-w-4xl px-4 py-6">{children}</main>
    </div>
  )
}
