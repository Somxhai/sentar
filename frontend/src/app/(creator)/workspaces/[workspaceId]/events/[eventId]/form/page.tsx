// app/(creator)/events/[eventId]/form/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { FormSchema } from "@/lib/types"
import Builder from "./builder"

export default async function FormPage({ params }: { params: { eventId: string } }) {
  const form = (await api.getForm(params.eventId)) as FormSchema
  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <Builder eventId={params.eventId} initialForm={form} />
    </div>
  )
}
