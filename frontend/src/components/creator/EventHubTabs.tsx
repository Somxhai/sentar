// components/creator/EventHubTabs.tsx
"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"

const tabs = [
  { href: (id: string) => `/events/${id}`, label: "Overview" },
  { href: (id: string) => `/events/${id}/form`, label: "Form" },
  { href: (id: string) => `/events/${id}/sections`, label: "Sections" },
  { href: (id: string) => `/events/${id}/objects`, label: "Objects" },
  { href: (id: string) => `/events/${id}/grid`, label: "Grid" },
  { href: (id: string) => `/events/${id}/reservations`, label: "Reservations" },
  { href: (id: string) => `/events/${id}/analytics`, label: "Analytics" },
  { href: (id: string) => `/events/${id}/preview`, label: "Preview" },
]

export default function EventHubTabs({ eventId }: { eventId: string }) {
  const pathname = usePathname()
  return (
    <div className="mb-6 flex flex-wrap gap-2">
      {tabs.map((t) => {
        const href = t.href(eventId)
        const active = pathname === href
        return (
          <Link
            key={t.label}
            href={href}
            className={`rounded-md border px-3 py-1.5 text-sm ${active ? "bg-accent" : "hover:bg-accent"}`}
          >
            {t.label}
          </Link>
        )
      })}
    </div>
  )
}
