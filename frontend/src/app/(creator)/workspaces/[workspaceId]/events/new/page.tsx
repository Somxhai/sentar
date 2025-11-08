// app/(creator)/workspaces/[workspaceId]/events/new/page.tsx
"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { api } from "@/lib/api"

export default function NewEventPage({ params }: { params: { workspaceId: string } }) {
  const router = useRouter()
  const [title, setTitle] = useState("")
  const [description, setDescription] = useState("")
  const [startsAt, setStartsAt] = useState("")
  const [endsAt, setEndsAt] = useState("")
  const [loading, setLoading] = useState(false)
  const [err, setErr] = useState<string | null>(null)

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true); setErr(null)
    try {
      const res = await api.createEvent({
        workspaceId: params.workspaceId,
        title,
        description,
        startsAt: startsAt || undefined,
        endsAt: endsAt || undefined,
      }) as { id: string }
      router.replace(`/events/${res.id}`)
    } catch (e: any) {
      setErr(e?.message ?? "Create failed")
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="mx-auto max-w-lg">
      <h1 className="mb-4 text-xl font-semibold">New Event</h1>
      <form onSubmit={submit} className="space-y-4">
        <label className="block">
          <div className="mb-1 text-sm">Title</div>
          <input className="w-full rounded border px-3 py-2 text-sm" value={title} onChange={e=>setTitle(e.target.value)} />
        </label>
        <label className="block">
          <div className="mb-1 text-sm">Description</div>
          <textarea className="w-full rounded border px-3 py-2 text-sm" value={description} onChange={e=>setDescription(e.target.value)} />
        </label>
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label className="block">
            <div className="mb-1 text-sm">Starts at</div>
            <input type="datetime-local" className="w-full rounded border px-3 py-2 text-sm" value={startsAt} onChange={e=>setStartsAt(e.target.value)} />
          </label>
          <label className="block">
            <div className="mb-1 text-sm">Ends at</div>
            <input type="datetime-local" className="w-full rounded border px-3 py-2 text-sm" value={endsAt} onChange={e=>setEndsAt(e.target.value)} />
          </label>
        </div>

        {err && <p className="text-sm text-destructive">{err}</p>}

        <button disabled={loading} className="rounded border px-3 py-2 text-sm hover:bg-accent">
          {loading ? "Creating..." : "Create"}
        </button>
      </form>
    </div>
  )
}
