// app/(creator)/workspaces/[workspaceId]/page.tsx
import Link from "next/link"
import { api } from "@/lib/api"
import type { Event, Workspace } from "@/lib/types"

export default async function WorkspacePage({ params }: { params: { workspaceId: string } }) {
  const [ws, events] = await Promise.all([
    api.getWorkspace(params.workspaceId) as Promise<Workspace>,
    api.getWorkspaceEvents(params.workspaceId) as Promise<Event[]>,
  ])

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-semibold">{ws.name}</h1>
        <Link
          href={`/workspaces/${ws.id}/events/new`}
          className="rounded-md border px-3 py-1.5 text-sm hover:bg-accent"
        >
          + New Event
        </Link>
      </div>

      {events.length === 0 ? (
        <div className="rounded-lg border p-8 text-center text-sm text-muted-foreground">
          ยังไม่มี Event — สร้างอันแรกเลย
        </div>
      ) : (
        <ul className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          {events.map((ev) => (
            <li key={ev.id} className="rounded-lg border p-4">
              <div className="mb-1 text-base font-medium">{ev.title}</div>
              <div className="text-xs text-muted-foreground">{ev.description}</div>
              <div className="mt-3 flex gap-2">
                <Link href={`/events/${ev.id}`} className="rounded-md border px-3 py-1.5 text-sm hover:bg-accent">
                  Open
                </Link>
                <Link href={`/events/${ev.id}/preview`} className="rounded-md border px-3 py-1.5 text-sm hover:bg-accent">
                  Preview
                </Link>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
