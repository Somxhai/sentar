// app/(creator)/events/[eventId]/reservations/page.tsx
import EventHubTabs from "@/components/creator/EventHubTabs"
import { api } from "@/lib/api"
import type { Reservation } from "@/lib/types"

export default async function ReservationsPage({ params }: { params: { eventId: string } }) {
  const list = (await api.getReservations(params.eventId)) as Reservation[]

  return (
    <div className="space-y-6">
      <EventHubTabs eventId={params.eventId} />
      <div className="rounded-lg border p-4">
        <div className="mb-2 text-sm font-medium">Reservations</div>
        <div className="overflow-x-auto">
          <table className="w-full text-left text-sm">
            <thead className="border-b text-xs text-muted-foreground">
              <tr>
                <th className="py-2">ID</th><th>User</th><th>Status</th><th>Total</th><th>Expires</th><th>Items</th>
              </tr>
            </thead>
            <tbody>
              {list.map((r) => (
                <tr key={r.id} className="border-b last:border-0">
                  <td className="py-2">{r.id.slice(0,8)}</td>
                  <td className="text-xs">{r.userId}</td>
                  <td>{r.status}</td>
                  <td>฿{r.totalPrice}</td>
                  <td className="text-xs">{r.expiresAt ?? "-"}</td>
                  <td className="text-xs">{r.items.map(i=>i.eventObjectId.slice(0,8)).join(", ")}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}
