'use client';
import { useRouter } from 'next/router';
import { MainLayout } from '@/app/main-layout';
import AssistantThread from '@/app/_components/assistant-thread';


export default function Page() {
  const router = useRouter();
  const { id } = router.query;

  return (
    <MainLayout>
      <AssistantThread threadId={`${id}`} />
    </MainLayout>
  );
}
