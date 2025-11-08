// lib/api.ts
const BASE = process.env.NEXT_PUBLIC_API_URL || ""

async function jfetch<T>(path: string, init?: RequestInit): Promise<T> {
  const r = await fetch(`${BASE}${path}`, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      ...(init?.headers || {}),
    },
    // ปล่อย cache control ให้ backend ตัดสินใจ; หน้า server ใช้ dynamic แล้ว
  })
  if (!r.ok) {
    let msg = r.statusText
    try {
      const j = await r.json()
      msg = j?.message || j?.error || msg
    } catch {}
    throw new Error(msg)
  }
  return r.json() as Promise<T>
}

export const api = {
  // ดึงรายการ workspace ของ user ปัจจุบัน
  getMyWorkspaces: () => jfetch<Array<{ id: string; name: string; ownerId: string }>>(`/my/workspaces`),

  // สร้าง workspace ใหม่
  createWorkspace: (name: string) =>
    jfetch<{ id: string; name: string; ownerId: string }>(`/workspace`, {
      method: "POST",
      body: JSON.stringify({ name }),
    }),
}
