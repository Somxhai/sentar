// app/(creator)/events/[eventId]/page.tsx
import { api } from "@/lib/api"
import type { Event } from "@/lib/types"
import EventHubTabs from "@/components/creator/EventHubTabs"

export default async function EventHub({ params }: { params: { eventId: string } }) {
  const ev = (await api.getEvent(params.eventId)) as Event
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-xl font-semibold">{ev.title}</h1>
        {ev.description && <p className="text-sm text-muted-foreground">{ev.description}</p>}
      </div>

      <EventHubTabs eventId={ev.id} />

      <div className="rounded-lg border p-6">
        <div className="text-sm text-muted-foreground">
          เลือกแท็บด้านบนเพื่อจัดการ Form, Sections, Objects, Grid, Reservations และ Analytics
        </div>
      </div>
    </div>
  )
}
