'use client';

import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { OnThreadDelete, ThreadPreview, ThreadPreviewComponentDto } from './thread-preview';
import { cn } from '@/lib/utils';
import { ReactNode } from 'react';


interface Props<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto> {
  threads?: TThread[]
  error?: Error
  isLoading? : boolean

  className?: string

  children?: ReactNode

  onDelete?: OnThreadDelete<TThread>
}

export function ThreadsPreviewCollection<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto>({ error, threads, isLoading, className, children, onDelete }: Props<TThread>) {
  return (
    <div className={cn('w-full flex flex-col items-center gap-1 overflow-y-auto pb-4', className)}>
      <div className='w-full flex justify-start gap-4'>
        {children}
        { threads?.map((thread) => (
          <ThreadPreview key={thread.id} thread={thread} onDelete={onDelete} />
        ))}
        { error && <ErrorDetails error={error}/>}
      </div>
      { isLoading && <ThreeDotsLoading/> }
    </div>
  );
}
