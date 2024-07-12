'use client';
import { useRouter } from 'next/router';
import AssistantThread from '@/app/_components/assistant-thread';
import RecentThreads from '@/app/_components/recent-threads';


export default function Page() {
  const router = useRouter();

  const id = typeof router.query.id === 'string' ? router.query.id : undefined;
  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      { id && <AssistantThread threadId={id} /> }
    </main>
  );
}
