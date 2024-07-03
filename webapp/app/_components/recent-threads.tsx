'use client';
import { ThreadsPreviewCollection } from '@/components/threads-preview-collection';
import { useRecentThreads } from '@/lib/use-recent-threads';


export default function RecentThreads() {
  const { data, error, isLoading } = useRecentThreads();

  return (
    <section className='w-full p-2'>
      <ThreadsPreviewCollection error={error} isLoading={isLoading} threads={data} />
    </section>
  );
}
