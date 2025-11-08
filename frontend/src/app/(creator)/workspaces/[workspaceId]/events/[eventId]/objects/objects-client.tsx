// app/(creator)/events/[eventId]/objects/objects-client.tsx
"use client"

import { useState } from "react"
import { api } from "@/lib/api"
import type { EventObject } from "@/lib/types"

export default function ObjectsClient({ initial }: { initial: EventObject[] }) {
  const [list, setList] = useState(initial)

  const toggle = async (id: string, next: Partial<EventObject>) => {
    const updated = await api.patchObject(id, next) as Partial<EventObject>
    setList((prev) => prev.map((o) => (o.id === id ? { ...o, ...updated } : o)))
  }

  return (
    <div className="rounded-lg border p-4">
      <div className="mb-2 text-sm font-medium">Event Objects</div>
      <div className="overflow-x-auto">
        <table className="w-full text-left text-sm">
          <thead className="border-b text-xs text-muted-foreground">
            <tr>
              <th className="py-2">Label</th><th>Type</th><th>Section</th><th>Status</th><th>Enable</th><th></th>
            </tr>
          </thead>
          <tbody>
            {list.map((o) => (
              <tr key={o.id} className="border-b last:border-0">
                <td className="py-2">{o.label}</td>
                <td>{o.objectType}</td>
                <td className="text-xs">{o.sectionId ?? "-"}</td>
                <td>{o.status}</td>
                <td>{o.isEnable ? "true" : "false"}</td>
                <td>
                  <div className="flex gap-2">
                    <button className="rounded border px-2 py-1 text-xs" onClick={() => toggle(o.id, { isEnable: !o.isEnable })}>
                      Toggle Enable
                    </button>
                    <button className="rounded border px-2 py-1 text-xs" onClick={() => toggle(o.id, { status: o.status === "available" ? "booked" : "available" })}>
                      Toggle Status
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
