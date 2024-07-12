
import { OnThreadDelete } from '@/components/thread-preview';
import { CreateNewThread } from './create-new-thread';
import { ThreadsPreviewCollection } from '@/components/threads-preview-collection';

import { useCallback } from 'react';
import { useDeleteThread } from '@/lib/use-delete-thread';
import { useListThreads } from '@/lib/use-list-threads';


export default function RecentThreads() {
  const { data, error, isLoading } = useListThreads();
  const deleteThread = useDeleteThread();
  const onDelete = useCallback<OnThreadDelete>((thread) => {
    deleteThread(thread.id);
  }, [deleteThread]);

  return (
    <section className='w-full p-2'>
      <ThreadsPreviewCollection error={error} isLoading={isLoading} threads={data} onDelete={onDelete}>
        <CreateNewThread />
      </ThreadsPreviewCollection>
    </section>
  );
}
