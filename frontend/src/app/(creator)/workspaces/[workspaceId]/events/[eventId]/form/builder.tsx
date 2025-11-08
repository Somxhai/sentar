// app/(creator)/events/[eventId]/form/builder.tsx
"use client"

import { useState } from "react"
import { api } from "@/lib/api"
import type { FormSchema } from "@/lib/types"

export default function Builder({ eventId, initialForm }: { eventId: string; initialForm: FormSchema }) {
  const [title, setTitle] = useState(initialForm?.title ?? "Attendee Info")
  const [description, setDescription] = useState(initialForm?.description ?? "")
  const [fieldsJSON, setFieldsJSON] = useState(JSON.stringify(initialForm?.schema ?? { fields: [] }, null, 2))
  const [saving, setSaving] = useState(false)
  const [msg, setMsg] = useState<string | null>(null)

  const save = async () => {
    setSaving(true); setMsg(null)
    try {
      const schema = JSON.parse(fieldsJSON)
      await api.saveForm(eventId, { title, description, schema })
      setMsg("Saved")
    } catch (e: any) {
      setMsg(e?.message ?? "Save failed")
    } finally { setSaving(false) }
  }

  return (
    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
      <div className="space-y-4">
        <label className="block">
          <div className="mb-1 text-sm">Form title</div>
          <input className="w-full rounded border px-3 py-2 text-sm" value={title} onChange={e=>setTitle(e.target.value)} />
        </label>
        <label className="block">
          <div className="mb-1 text-sm">Description</div>
          <textarea className="h-24 w-full rounded border px-3 py-2 text-sm" value={description} onChange={e=>setDescription(e.target.value)} />
        </label>
        <label className="block">
          <div className="mb-1 text-sm">Schema (JSON)</div>
          <textarea className="h-72 w-full rounded border px-3 py-2 font-mono text-xs"
            value={fieldsJSON} onChange={e=>setFieldsJSON(e.target.value)} />
        </label>
        <div className="flex items-center gap-3">
          <button onClick={save} disabled={saving} className="rounded border px-3 py-2 text-sm hover:bg-accent">
            {saving ? "Saving..." : "Save"}
          </button>
          {msg && <span className="text-sm text-muted-foreground">{msg}</span>}
        </div>
      </div>

      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Preview</div>
        {/* พรีวิวอย่างง่าย แสดง labels */}
        <ul className="space-y-2">
          {(() => {
            try {
              const s = JSON.parse(fieldsJSON)
              return (s.fields || []).map((f: any, idx: number) => (
                <li key={idx} className="rounded border p-2 text-sm">
                  <div className="font-medium">{f.label}</div>
                  <div className="text-xs text-muted-foreground">{f.type}{f.required ? " • required" : ""}</div>
                </li>
              ))
            } catch { return <li className="text-sm text-destructive">Invalid JSON</li> }
          })()}
        </ul>
      </div>
    </div>
  )
}
