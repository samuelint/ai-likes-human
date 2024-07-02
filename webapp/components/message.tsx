import { twMerge } from 'tailwind-merge';
import { ReactNode } from 'react';
import { ChatMessageContentDto } from './chat.type';
import { MessageContent } from './message-content';


export interface ChatMessageProps {
  className?: string
  content?: ChatMessageContentDto[]
  children?: ReactNode
  type?: 'ai' | 'user'
  actions?: ReactNode
}

export function Message({ className, content, children, actions, type }: ChatMessageProps) {
  return (
    <div className={twMerge('group', className)}>
      <div className={twMerge(
        'flex flex-col gap-2 rounded-lg px-3 py-2 text-sm bg-white dark:bg-gray-700 dark:text-white',
        type === 'user' && 'bg-slate-100',
      )}
      >
        { content?.map((contentItem, index) => <MessageContent key={index} content={contentItem} /> ) }
        { children }
      </div>
      { actions && <div className={twMerge('w-full flex justify-end relative', 'opacity-0 group-hover:opacity-100 transition-opacity duration-300')}>
        <div className='w-fit flex gap-2 bg-slate-200/90 rounded px-2 py-1 -mt-1 mr-1'>{actions}</div>
      </div>
      }
    </div>
  );
}
