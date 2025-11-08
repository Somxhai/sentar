// app/api/auth/[...all]/route.ts
import { auth } from "@/lib/auth"
import { toNextJsHandler } from "better-auth/next-js"

// map ทุกเมธอดไปที่ better-auth handler
export const { GET, POST } = toNextJsHandler(auth)
