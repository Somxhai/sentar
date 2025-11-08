// app/(creator)/events/[eventId]/sections/sections-client.tsx
"use client"

import { useState } from "react"
import { api } from "@/lib/api"
import type { Section } from "@/lib/types"

export default function SectionsClient({ eventId, initial }: { eventId: string; initial: Section[] }) {
  const [list, setList] = useState(initial)
  const [title, setTitle] = useState("")
  const [price, setPrice] = useState<number | ''>("")
  const [saving, setSaving] = useState(false)

  const add = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!title || price === '') return
    setSaving(true)
    try {
      const s = await api.createSection(eventId, { title, price: Number(price) }) as Section
      setList([...list, s])
      setTitle(""); setPrice("")
    } catch (e) { console.error(e) } finally { setSaving(false) }
  }

  return (
    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Add Section</div>
        <form onSubmit={add} className="space-y-3">
          <label className="block">
            <div className="mb-1 text-sm">Title</div>
            <input className="w-full rounded border px-3 py-2 text-sm" value={title} onChange={e=>setTitle(e.target.value)} />
          </label>
          <label className="block">
            <div className="mb-1 text-sm">Price</div>
            <input type="number" className="w-full rounded border px-3 py-2 text-sm" value={price} onChange={e=>setPrice(e.target.value === '' ? '' : Number(e.target.value))} />
          </label>
          <button disabled={saving} className="rounded border px-3 py-2 text-sm hover:bg-accent">
            {saving ? "Saving..." : "Add"}
          </button>
        </form>
      </div>

      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Sections</div>
        <ul className="space-y-2">
          {list.map((s) => (
            <li key={s.id} className="flex items-center justify-between rounded border p-2 text-sm">
              <div>
                <div className="font-medium">{s.title}</div>
                <div className="text-xs text-muted-foreground">฿{s.price}</div>
              </div>
              <span className="rounded bg-muted px-2 py-0.5 text-xs">{s.id.slice(0,8)}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  )
}
