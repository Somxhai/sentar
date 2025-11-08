// app/(creator)/events/[eventId]/grid/grid-client.tsx
"use client"

import { useState } from "react"
import { api } from "@/lib/api"
import type { EventGrid } from "@/lib/types"

export default function GridClient({ eventId, initial }: { eventId: string; initial: EventGrid }) {
  const [json, setJson] = useState(JSON.stringify(initial?.grid ?? { rows:10, cols:10, objects: [] }, null, 2))
  const [saving, setSaving] = useState(false)
  const [msg, setMsg] = useState<string | null>(null)

  const save = async () => {
    setSaving(true); setMsg(null)
    try {
      const parsed = JSON.parse(json)
      await api.saveGrid(eventId, parsed)
      setMsg("Saved")
    } catch (e: any) {
      setMsg(e?.message ?? "Save failed")
    } finally { setSaving(false) }
  }

  return (
    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
      <div>
        <div className="mb-1 text-sm">Grid (JSON)</div>
        <textarea className="h-96 w-full rounded border px-3 py-2 font-mono text-xs" value={json} onChange={e=>setJson(e.target.value)} />
        <div className="mt-3 flex items-center gap-3">
          <button onClick={save} disabled={saving} className="rounded border px-3 py-2 text-sm hover:bg-accent">
            {saving ? "Saving..." : "Save"}
          </button>
          {msg && <span className="text-sm text-muted-foreground">{msg}</span>}
        </div>
      </div>

      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Preview (rough)</div>
        {(() => {
          try {
            const g = JSON.parse(json)
            return (
              <div className="inline-grid border" style={{ gridTemplateColumns: `repeat(${g.cols}, 20px)` }}>
                {Array.from({ length: g.rows * g.cols }).map((_, i) => {
                  const x = (i % g.cols) + 1
                  const y = Math.floor(i / g.cols) + 1
                  const obj = (g.objects || []).find((o: any) => o.x === x && o.y === y)
                  return (
                    <div key={i} className={`h-5 w-5 border ${obj ? "bg-foreground/20" : "bg-muted"}`} title={obj?.objectId ?? ""}/>
                  )
                })}
              </div>
            )
          } catch { return <div className="text-sm text-destructive">Invalid JSON</div> }
        })()}
      </div>
    </div>
  )
}
