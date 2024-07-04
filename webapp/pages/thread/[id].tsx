'use client';
import { useRouter } from 'next/router';
import { MainLayout } from '@/app/main-layout';
import AssistantThread from '@/app/_components/assistant-thread';
import RecentThreads from '@/app/_components/recent-threads';


export default function Page() {
  const router = useRouter();
  const { id } = router.query;

  return (
    <MainLayout>
      <main className="h-full flex flex-col">
        <RecentThreads />
        <AssistantThread threadId={`${id}`} />
      </main>
    </MainLayout>
  );
}
