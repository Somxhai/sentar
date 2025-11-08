// app/(creator)/workspaces/page.tsx
import Link from "next/link"
import { api } from "@/lib/api"
import { Workspace } from "@/lib/types"

export const dynamic = "force-dynamic" // ให้ดึงสดเวลา dev/preview

export default async function Page() {
  let workspaces: Workspace[] = []
  try {
    workspaces = await api.getMyWorkspaces() as Workspace[]
  } catch (e) {
    // เงียบ ๆ ไว้ก่อน แล้วโชว์ empty state ด้านล่าง
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-semibold">Your Workspaces</h1>
        <Link
          href="/workspaces/new"
          className="inline-flex items-center rounded-md border px-3 py-1.5 text-sm hover:bg-accent"
        >
          + Create
        </Link>
      </div>

      {workspaces.length === 0 ? (
        <div className="rounded-lg border p-8 text-center text-sm text-muted-foreground">
          ยังไม่มี workspace — เริ่มเลยที่{" "}
          <Link href="/workspaces/new" className="underline">
            สร้าง workspace
          </Link>
        </div>
      ) : (
        <ul className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          {workspaces.map((w) => (
            <li key={w.id} className="rounded-lg border p-4">
              <div className="mb-2 flex items-center justify-between gap-2">
                <h2 className="truncate text-base font-medium">{w.name}</h2>
                <span className="rounded bg-muted px-2 py-0.5 text-xs">
                  {w.id.slice(0, 8)}
                </span>
              </div>
              <div className="mt-3 flex gap-2">
                <Link
                  href={`/workspaces/${w.id}`}
                  className="inline-flex items-center rounded-md border px-3 py-1.5 text-sm hover:bg-accent"
                >
                  Open
                </Link>
                <Link
                  href={`/workspaces/${w.id}/events/new`}
                  className="inline-flex items-center rounded-md border px-3 py-1.5 text-sm hover:bg-accent"
                >
                  + New Event
                </Link>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}
