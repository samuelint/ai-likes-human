'use client';

import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { ThreadPreviewDto } from '@/lib/thread.type';
import { ThreadPreview } from './thread-preview';
import { cn } from '@/lib/utils';
import { ReactNode } from 'react';


interface Props {
  threads?: ThreadPreviewDto[]
  error?: Error
  isLoading? : boolean

  className?: string

  children?: ReactNode
}

export function ThreadsPreviewCollection({ error, threads, isLoading, className, children }: Props) {
  return (
    <div className={cn('w-full flex flex-col items-center gap-1 overflow-y-auto', className)}>
      <div className='w-full flex justify-start gap-4'>
        {children}
        { threads?.map((thread) => (
          <ThreadPreview key={thread.id} thread={thread} />
        ))}
        { error && <ErrorDetails error={error}/>}
      </div>
      { isLoading && <ThreeDotsLoading/> }
    </div>
  );
}
