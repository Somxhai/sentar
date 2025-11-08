// src/lib/api.ts
const BASE = process.env.NEXT_PUBLIC_API_URL || ""

async function jfetch<T>(path: string, init?: RequestInit): Promise<T> {
  const r = await fetch(`${BASE}${path}`, {
    ...init,
    headers: { "Content-Type": "application/json", ...(init?.headers || {}) },
    cache: "no-store",
  })
  if (!r.ok) {
    let msg = `${r.status} ${r.statusText}`
    try {
      const j = await r.json()
      msg = j?.message || j?.error || msg
    } catch {}
    throw new Error(msg)
  }
  return r.json() as Promise<T>
}

export const api = {
  // Workspaces
  getWorkspace: (id: string) => jfetch(`/workspace/${id}`),
  getMyWorkspaces: () => jfetch(`/my/workspaces`),
  createWorkspace: (name: string) =>
    jfetch(`/workspace`, { method: "POST", body: JSON.stringify({ name }) }),

  // Events
  getEvent: (id: string) => jfetch(`/event/${id}`),
  createEvent: (body: any) =>
    jfetch(`/event`, { method: "POST", body: JSON.stringify(body) }),
  getWorkspaceEvents: (workspaceId: string) =>
    jfetch(`/workspace/${workspaceId}/events`),

  // Sections
  getSections: (eventId: string) => jfetch(`/events/${eventId}/sections`),
  createSection: (
    eventId: string,
    body: { title: string; price: number }
  ) =>
    jfetch(`/events/${eventId}/sections`, {
      method: "POST",
      body: JSON.stringify(body),
    }),

  // Event Objects
  getObjects: (eventId: string) => jfetch(`/events/${eventId}/objects`),
  patchObject: (id: string, body: any) =>
    jfetch(`/event-objects/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    }),

  // Grid
  getGrid: (eventId: string) => jfetch(`/events/${eventId}/grid`),
  saveGrid: (eventId: string, grid: any) =>
    jfetch(`/events/${eventId}/grid`, {
      method: "POST",
      body: JSON.stringify({ grid }),
    }),

  // Form
  getForm: (eventId: string) => jfetch(`/events/${eventId}/form`),
  saveForm: (eventId: string, payload: any) =>
    jfetch(`/events/${eventId}/form`, {
      method: "POST",
      body: JSON.stringify(payload),
    }),

  // Reservations
  createReservation: (body: any) =>
    jfetch(`/reservations`, { method: "POST", body: JSON.stringify(body) }),
  patchReservation: (id: string, body: any) =>
    jfetch(`/reservations/${id}`, {
      method: "PATCH",
      body: JSON.stringify(body),
    }),
  getReservation: (id: string) => jfetch(`/reservations/${id}`),

  // Analytics
  getReservations: (eventId: string) =>
    jfetch(`/events/${eventId}/reservations`),
  getEventAnalytics: (eventId: string) =>
    jfetch(`/events/${eventId}/analytics`),
}
