// components/responder/FormRunner.tsx
"use client"

import { useMemo, useState } from "react"
import { useRouter } from "next/navigation"
import { api } from "@/lib/api"
import type { FormSchema, EventObject, EventGrid, Section } from "@/lib/types"
import SeatPicker from "./SeatPicker"
import HoldTimer from "./HoldTimer"
import { getAnonId } from "@/lib/anon"

type Props = {
  eventId: string
  form: FormSchema
  objects: EventObject[]
  grid: EventGrid
  sections: Section[]
}

export default function FormRunner({ eventId, form, objects, grid, sections }: Props) {
  const router = useRouter()
  const anonId = getAnonId()
  const [answers, setAnswers] = useState<Record<string, any>>({})
  const [selectedObjectIds, setSelectedObjectIds] = useState<string[]>([])
  const [reservationId, setReservationId] = useState<string | null>(null)
  const [expiresAt, setExpiresAt] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)
  const [err, setErr] = useState<string | null>(null)

  const priceBySection = useMemo(() => {
    const map = new Map<string, number>()
    sections.forEach((s) => map.set(s.id, s.price))
    return map
  }, [sections])

  const hold = async () => {
    if (selectedObjectIds.length === 0) return setErr("ยังไม่ได้เลือกที่นั่ง")
    setBusy(true); setErr(null)
    try {
      const items = selectedObjectIds.map((id) => {
        const obj = objects.find((o) => o.id === id)!
        const price = obj.sectionId ? (priceBySection.get(obj.sectionId) ?? 0) : 0
        return { eventObjectId: id, price }
      })
      const r = await api.createReservation({
        userId: anonId,
        eventId,
        items,
      }) as { id: string; expiresAt?: string }
      setReservationId(r.id)
      setExpiresAt(r.expiresAt ?? null)
    } catch (e: any) {
      setErr(e?.message ?? "Hold failed")
    } finally {
      setBusy(false)
    }
  }

  const confirm = async () => {
    if (!reservationId) return setErr("ยังไม่มีการ hold ที่นั่ง")
    setBusy(true); setErr(null)
    try {
      await api.patchReservation(reservationId, { status: "confirmed" })
      // ปกติจะ POST submissions ด้วย แต่ใน MVP นี้ยืนยันจองอย่างเดียวก่อน
      router.push(`/r/${reservationId}`)
    } catch (e: any) {
      setErr(e?.message ?? "Confirm failed")
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
      {/* ฟอร์มข้อมูลพื้นฐาน */}
      <div className="space-y-4">
        <div className="rounded-lg border p-4">
          <div className="mb-2 text-sm font-medium">{form.title}</div>
          <div className="space-y-3">
            {(form.schema?.fields ?? []).map((f, idx) => (
              <label key={idx} className="block">
                <div className="mb-1 text-sm">{f.label}{f.required ? " *" : ""}</div>
                {f.type === "text" || f.type === "email" || f.type === "number" ? (
                  <input
                    type={f.type}
                    className="w-full rounded border px-3 py-2 text-sm"
                    onChange={(e) =>
                      setAnswers((a) => ({ ...a, [f.label]: e.target.value }))
                    }
                  />
                ) : f.type === "dropdown" ? (
                  <select
                    className="w-full rounded border px-3 py-2 text-sm"
                    onChange={(e) =>
                      setAnswers((a) => ({ ...a, [f.label]: e.target.value }))
                    }
                  >
                    <option value="">-- เลือก --</option>
                    {(f.options ?? []).map((opt) => (
                      <option key={opt} value={opt}>{opt}</option>
                    ))}
                  </select>
                ) : null}
              </label>
            ))}
          </div>
        </div>

        <div className="flex items-center gap-3">
          <button onClick={confirm} disabled={busy || !reservationId} className="rounded border px-3 py-2 text-sm hover:bg-accent disabled:opacity-60">
            {busy ? "Processing..." : "Confirm"}
          </button>
          {err && <span className="text-sm text-destructive">{err}</span>}
        </div>
      </div>

      {/* เลือกที่นั่ง */}
      <div className="space-y-3">
        <div className="rounded-lg border p-4">
          <div className="mb-2 text-sm font-medium">เลือกที่นั่ง</div>
          <SeatPicker
            objects={objects}
            grid={grid}
            selected={selectedObjectIds}
            onChange={setSelectedObjectIds}
          />
          <div className="mt-3 flex items-center gap-3">
            <button onClick={hold} disabled={busy || selectedObjectIds.length === 0} className="rounded border px-3 py-2 text-sm hover:bg-accent disabled:opacity-60">
              {busy ? "Holding..." : "Hold seats"}
            </button>
            {reservationId && expiresAt && <HoldTimer expiresAt={expiresAt} />}
          </div>
        </div>
      </div>
    </div>
  )
}
