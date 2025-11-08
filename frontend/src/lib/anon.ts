// lib/anon.ts
export function getAnonId(): string {
  if (typeof window === "undefined") return "anonymous"
  const k = "sentar_anon_id"
  let v = localStorage.getItem(k)
  if (!v) {
    v = (crypto?.randomUUID?.() ?? Math.random().toString(36).slice(2)) as string
    localStorage.setItem(k, v)
  }
  return v
}
