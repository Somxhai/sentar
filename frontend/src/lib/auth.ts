// lib/auth.ts
import { betterAuth } from "better-auth"
// ถ้าใช้ Prisma/Postgres (แนะนำ):
// import { prismaAdapter } from "better-auth/adapters/prisma"
// import { prisma } from "./prisma"

export const auth = betterAuth({
  // database: prismaAdapter(prisma, { provider: "postgresql" }), // เปิดถ้าต่อ DB ด้วย Prisma
  baseURL: process.env.BETTER_AUTH_URL, // สำหรับลิงก์ callback/อีเมล
  secret: process.env.BETTER_AUTH_SECRET,
  emailAndPassword: {
    enabled: true, // เปิด email/password auth
    autoSignIn: true, // สมัครเสร็จล็อกอินให้เลย
    minPasswordLength: 8,
  },
  session: {
    // ใช้ cookie-based session
    expiresIn: 60 * 60 * 24 * 7, // 7 วัน
    updateAge: 60 * 60 * 24,     // refresh อายุทุก 24 ชม.
  },
  // trustedOrigins: ["http://localhost:3000"], // เพิ่ม origin FE ถ้าต่างโดเมน
})
