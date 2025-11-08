// app/(creator)/events/[eventId]/preview/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"

export default function PreviewPage({ params }: { params: { eventId: string } }) {
  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Responder preview</div>
        <iframe
          src={`/e/${params.eventId}`}
          className="h-[75vh] w-full rounded border"
        />
      </div>
    </div>
  )
}
