// app/(creator)/events/[eventId]/grid/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { EventGrid } from "@/lib/types"
import GridClient from "./grid-client"

export default async function GridPage({ params }: { params: { eventId: string } }) {
  const grid = (await api.getGrid(params.eventId)) as EventGrid
  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <GridClient eventId={params.eventId} initial={grid} />
    </div>
  )
}
