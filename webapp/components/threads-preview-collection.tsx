'use client';

import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { ThreadPreviewDto } from '@/lib/thread.type';
import { ThreadPreview } from './thread-preview';
import { cn } from '@/lib/utils';


interface Props {
  threads?: ThreadPreviewDto[]
  error?: Error
  isLoading? : boolean

  className?: string
}

export function ThreadsPreviewCollection({ error, threads, isLoading, className }: Props) {
  return (
    <div className={cn('w-full flex flex-col items-center gap-1', className)}>
      <div className='w-full flex justify-start gap-4'>
        { threads?.map((thread) => (
          <ThreadPreview key={thread.id} thread={thread} />
        ))}
      </div>
      { isLoading && <ThreeDotsLoading/> }
      { error && <ErrorDetails error={error}/>}
    </div>
  );
}
