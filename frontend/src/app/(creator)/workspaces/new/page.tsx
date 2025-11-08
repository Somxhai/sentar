// app/(creator)/workspaces/new/page.tsx
"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { api } from "@/lib/api"

export default function NewWorkspacePage() {
  const router = useRouter()
  const [name, setName] = useState("")
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name.trim()) return setError("กรอกชื่อ workspace ก่อนนะ")
    setLoading(true)
    setError(null)
    try {
      const res = await api.createWorkspace(name.trim()) as { id: string }
      router.push(`/workspaces/${res.id}`)
    } catch (err: any) {
      setError(err?.message ?? "สร้างไม่สำเร็จ")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="mx-auto max-w-lg space-y-6">
      <h1 className="text-xl font-semibold">Create Workspace</h1>

      <form onSubmit={onSubmit} className="space-y-4">
        <label className="block">
          <div className="mb-1 text-sm text-muted-foreground">Name</div>
          <input
            className="block w-full rounded-md border px-3 py-2 text-sm outline-none focus:ring-2"
            placeholder="Event Hub"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>

        {error ? (
          <p className="text-sm text-destructive">{error}</p>
        ) : null}

        <button
          type="submit"
          disabled={loading}
          className="inline-flex items-center rounded-md border px-3 py-1.5 text-sm hover:bg-accent disabled:opacity-60"
        >
          {loading ? "Creating..." : "Create"}
        </button>
      </form>
    </div>
  )
}
