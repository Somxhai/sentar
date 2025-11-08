// app/(creator)/events/[eventId]/analytics/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { AnalyticsSummary } from "@/lib/types"

export default async function AnalyticsPage({ params }: { params: { eventId: string } }) {
  const data = (await api.getEventAnalytics(params.eventId)) as AnalyticsSummary

  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />

      <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
        {Object.entries(data.totals || {}).map(([k, v]) => (
          <div key={k} className="rounded-lg border p-4">
            <div className="text-xs uppercase text-muted-foreground">{k}</div>
            <div className="text-xl font-semibold">{v ?? 0}</div>
          </div>
        ))}
      </div>

      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">By Section</div>
        <table className="w-full text-left text-sm">
          <thead className="border-b text-xs text-muted-foreground">
            <tr><th className="py-2">Section</th><th>Sold</th><th>Revenue</th></tr>
          </thead>
          <tbody>
            {(data.bySection || []).map((s) => (
              <tr key={s.sectionId} className="border-b last:border-0">
                <td className="py-2">{s.title}</td>
                <td>{s.sold}</td>
                <td>฿{s.revenue}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
