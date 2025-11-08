// app/(creator)/events/[eventId]/sections/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { Section } from "@/lib/types"
import SectionsClient from "./sections-client"

export default async function SectionsPage({ params }: { params: { eventId: string } }) {
  const sections = (await api.getSections(params.eventId)) as Section[]
  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <SectionsClient eventId={params.eventId} initial={sections} />
    </div>
  )
}
