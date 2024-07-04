'use client';
import { NewThread } from '@/components/new-thread';
import { useCreateThread } from '@/lib/use-create-thread';
import { useRouter } from 'next/navigation';
import { useCallback } from 'react';


export function CreateNewThread() {
  const router = useRouter();
  const createThread = useCreateThread();

  const onClick = useCallback(async () => {
    const newThread = await createThread();
    router.push(`/thread/${newThread.id}`);
  }, [createThread, router]);

  return (
    <NewThread onClick={onClick} />
  );
}
