// lib/types.ts
export type Workspace = { id: string; name: string; ownerId: string }

export type Event = {
  id: string
  workspaceId: string
  title: string
  description?: string
  startsAt?: string
  endsAt?: string
}

export type Section = {
  id: string
  eventId: string
  title: string
  price: number
}

export type EventObject = {
  id: string
  objectType: "Seat" | "Table" | string
  eventId: string
  sectionId: string | null
  label: string
  isEnable: boolean
  status: "available" | "held" | "pending" | "booked"
}

export type EventGrid = {
  id: string
  eventId: string
  grid: {
    rows: number
    cols: number
    objects: Array<{ objectId: string; x: number; y: number }>
  }
}

export type FormSchema = {
  id: string
  eventId: string
  title: string
  description?: string
  schema: {
    fields: Array<{
      id?: string
      label: string
      type: "text" | "email" | "number" | "dropdown"
      required?: boolean
      options?: string[]
    }>
  }
}

export type ReservationItem = {
  id: string
  eventObjectId: string
  priceAtBooking: number
}

export type Reservation = {
  id: string
  userId: string
  eventId: string
  status: "held" | "confirmed" | "cancelled" | "expired"
  totalPrice: number
  expiresAt?: string
  items: ReservationItem[]
}

export type AnalyticsSummary = {
  totals: {
    views?: number
    starts?: number
    submissions?: number
    paid?: number
    revenue?: number
  }
  bySection?: Array<{ sectionId: string; title: string; sold: number; revenue: number }>
}
