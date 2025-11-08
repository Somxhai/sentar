// lib/auth-client.ts
"use client"

import { createAuthClient } from "better-auth/react"

// สามารถ export method ตรง ๆ ให้ import ง่าย
export const authClient = createAuthClient({
  // ถ้า backend auth อยู่โดเมนเดียว ไม่ต้องใส่ baseURL ก็ได้
  // baseURL: process.env.NEXT_PUBLIC_AUTH_URL, 
})

export const { signIn, signUp, signOut, useSession } = authClient
