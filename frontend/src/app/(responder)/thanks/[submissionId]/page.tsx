// app/(responder)/thanks/[submissionId]/page.tsx
export default function ThanksPage({ params }: { params: { submissionId: string } }) {
  return (
    <div className="rounded-lg border p-6">
      <h1 className="text-xl font-semibold">ขอบคุณ!</h1>
      <p className="mt-2 text-sm text-muted-foreground">
        เราได้รับแบบฟอร์มของคุณแล้ว • หมายเลขอ้างอิง:{" "}
        <span className="font-mono">{params.submissionId}</span>
      </p>
    </div>
  )
}
