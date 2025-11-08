// components/responder/HoldTimer.tsx
"use client"

import { useEffect, useState } from "react"

export default function HoldTimer({ expiresAt }: { expiresAt: string }) {
  const [remain, setRemain] = useState<number>(() => {
    const diff = new Date(expiresAt).getTime() - Date.now()
    return Math.max(0, Math.floor(diff / 1000))
  })

  useEffect(() => {
    const id = setInterval(() => {
      setRemain((r) => Math.max(0, r - 1))
    }, 1000)
    return () => clearInterval(id)
  }, [])

  const mm = String(Math.floor(remain / 60)).padStart(2, "0")
  const ss = String(remain % 60).padStart(2, "0")

  return (
    <span className={`rounded px-2 py-1 text-xs ${remain === 0 ? "bg-destructive/20 text-destructive" : "bg-muted"}`}>
      Hold expires in {mm}:{ss}
    </span>
  )
}
