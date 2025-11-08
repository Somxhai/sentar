// components/responder/SeatPicker.tsx
"use client"

import type { EventObject, EventGrid } from "@/lib/types"

type Props = {
  objects: EventObject[]
  grid: EventGrid
  selected: string[]
  onChange: (ids: string[]) => void
}

export default function SeatPicker({ objects, grid, selected, onChange }: Props) {
  // map objectId -> object
  const objById = new Map(objects.map(o => [o.id, o]))
  const enabledSet = new Set(objects.filter(o => o.isEnable && o.status === "available").map(o => o.id))

  const toggle = (objectId?: string) => {
    if (!objectId) return
    if (!enabledSet.has(objectId)) return
    const next = selected.includes(objectId)
      ? selected.filter((id) => id !== objectId)
      : [...selected, objectId]
    onChange(next)
  }

  const g = grid.grid ?? { rows: 10, cols: 10, objects: [] }

  // สร้างดัชนีตำแหน่ง -> objectId
  const at = (x: number, y: number) =>
    (g.objects || []).find((o: any) => o.x === x && o.y === y)?.objectId as string | undefined

  return (
    <div className="inline-grid border" style={{ gridTemplateColumns: `repeat(${g.cols}, 24px)` }}>
      {Array.from({ length: g.rows * g.cols }).map((_, i) => {
        const x = (i % g.cols) + 1
        const y = Math.floor(i / g.cols) + 1
        const objectId = at(x, y)
        const obj = objectId ? objById.get(objectId) : undefined
        const selectable = objectId && enabledSet.has(objectId)
        const isSelected = objectId ? selected.includes(objectId) : false

        return (
          <button
            key={i}
            type="button"
            title={obj?.label}
            onClick={() => toggle(objectId)}
            className={[
              "m-0.5 h-5 w-5 rounded border text-[10px]",
              selectable ? "hover:bg-foreground/10" : "bg-muted opacity-50 cursor-default",
              isSelected ? "bg-foreground/30" : ""
            ].join(" ")}
          />
        )
      })}
    </div>
  )
}
