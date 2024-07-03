import { type ThreadPreviewDto } from '@/lib/thread.type';
import Link from 'next/link';
import { buttonVariants } from './ui/button';
import { cn } from '@/lib/utils';


interface Props {
  thread: Pick<ThreadPreviewDto, 'id' | 'title' | 'created_at'>
}

export function ThreadPreview({ thread }: Props) {
  const { id, title, created_at } = thread;
  return (
    <Link href={`/thread/${id}`} className={cn(buttonVariants({ variant: 'outline' }), 'flex flex-col items-start')}>
      <span className=''>{title}</span>
      <span className='text-xs text-slate-400'>{created_at?.toLocaleDateString('en')}</span>
    </Link>
  );
}
