// app/(responder)/e/[eventId]/page.tsx
import { api } from "@/lib/api"
import type { Event, FormSchema, EventObject, EventGrid, Section } from "@/lib/types"
import FormRunner from "@/components/responder/FormRunner"

export default async function ResponderEventPage({ params }: { params: { eventId: string } }) {
  const eventId = params.eventId
  const [ev, form, objects, grid, sections] = await Promise.all([
    api.getEvent(eventId) as Promise<Event>,
    api.getForm(eventId) as Promise<FormSchema>,
    api.getObjects(eventId) as Promise<EventObject[]>,
    api.getGrid(eventId) as Promise<EventGrid>,
    api.getSections(eventId) as Promise<Section[]>,
  ])

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-xl font-semibold">{ev.title}</h1>
        {ev.description && <p className="text-sm text-muted-foreground">{ev.description}</p>}
      </div>

      <FormRunner
        eventId={eventId}
        form={form}
        objects={objects}
        grid={grid}
        sections={sections}
      />
    </div>
  )
}
