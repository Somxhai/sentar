// app/(responder)/r/[reservationId]/page.tsx
import { api } from "@/lib/api"
import type { Reservation } from "@/lib/types"
import Link from "next/link"

export default async function ReservationSummary({ params }: { params: { reservationId: string } }) {
  const r = (await api.getReservation(params.reservationId)) as Reservation

  return (
    <div className="space-y-6">
      <h1 className="text-xl font-semibold">Reservation</h1>

      <div className="rounded-lg border p-4 text-sm">
        <div className="mb-1 text-muted-foreground">Reservation ID</div>
        <div className="font-mono">{r.id}</div>

        <div className="mt-4 grid grid-cols-2 gap-4">
          <div>
            <div className="mb-1 text-muted-foreground">Status</div>
            <div className="font-medium">{r.status}</div>
          </div>
          <div>
            <div className="mb-1 text-muted-foreground">Total</div>
            <div className="font-medium">฿{r.totalPrice}</div>
          </div>
          <div>
            <div className="mb-1 text-muted-foreground">Event</div>
            <div className="font-mono">{r.eventId}</div>
          </div>
          <div>
            <div className="mb-1 text-muted-foreground">Expires at</div>
            <div className="font-mono">{r.expiresAt ?? "-"}</div>
          </div>
        </div>

        <div className="mt-4">
          <div className="mb-1 text-muted-foreground">Items</div>
          <ul className="space-y-1">
            {r.items.map((i) => (
              <li key={i.id} className="rounded border p-2">
                <div className="font-mono text-xs">{i.eventObjectId}</div>
                <div className="text-xs text-muted-foreground">฿{i.priceAtBooking}</div>
              </li>
            ))}
          </ul>
        </div>
      </div>

      <Link href={`/e/${r.eventId}`} className="inline-flex rounded border px-3 py-1.5 text-sm hover:bg-accent">
        Back to form
      </Link>
    </div>
  )
}
