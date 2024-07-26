import { Link } from 'wouter';
import { type ThreadPreviewDto } from '@/lib/assistant/thread.type';
import { cn } from '@/lib/utils';
import { buttonVariants } from './ui/button';
import { ThreadPreviewContextMenu } from './thread-preview-context-menu';
import { toFromNowFormattedDate } from '@/lib/date';


export type ThreadPreviewComponentDto = Pick<ThreadPreviewDto, 'id' | 'title' | 'created_at'>;
export type OnThreadDelete<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto> = (thread: TThread) => void;
interface Props<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto> {
  onDelete?: OnThreadDelete<TThread>
  thread: TThread
}

export function ThreadPreview<TThread extends ThreadPreviewComponentDto = ThreadPreviewComponentDto>({ thread, onDelete }: Props<TThread>) {
  const { id, title, created_at } = thread;
  const isActive = window.location.pathname.includes(`/thread/${id}`);

  return (
    <ThreadPreviewContextMenu onDelete={() => onDelete && onDelete(thread)}>
      <Link href={`/thread/${id}`} className={cn(buttonVariants({ variant: isActive ? 'secondary' : 'outline' }), 'flex flex-col items-start')}>
        <span className=''>{title}</span>
        <span className='text-xs text-slate-400'>{toFromNowFormattedDate(created_at)}</span>
      </Link>
    </ThreadPreviewContextMenu>
  );
}

