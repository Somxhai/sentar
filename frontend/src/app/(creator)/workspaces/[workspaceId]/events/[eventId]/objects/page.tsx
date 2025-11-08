// app/(creator)/events/[eventId]/objects/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { EventObject } from "@/lib/types"
import ObjectsClient from "./objects-client"

export default async function ObjectsPage({ params }: { params: { eventId: string } }) {
  const objects = (await api.getObjects(params.eventId)) as EventObject[]
  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <ObjectsClient initial={objects} />
    </div>
  )
}
