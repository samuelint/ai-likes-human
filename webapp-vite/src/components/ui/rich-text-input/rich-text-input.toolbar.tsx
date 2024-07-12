import { twMerge } from 'tailwind-merge';
import { CodeFormatToolbarActions } from './module/code-highlight';
import { HeadingToolbarActions } from './module/heading';
import { HistoryToolbarActions } from './module/history';
import { MarkdownToolbarActions } from './module/markdown';
import { TextFormatToolbarActions } from './module/text-format';
import { VerticalSeparator } from './toolbar';
import { PropsWithChildren, useRef } from 'react';


export type ToolbarType = 'complete' | 'minimal' | 'none';
interface Props extends PropsWithChildren {
  editable?: boolean
  className?: string
  type?: ToolbarType
}

export default function RichTextToolbar({ editable, type = 'complete', className, children }: Props) {
  const divRef = useRef<HTMLDivElement>(null);

  if (type === 'none') return <div className='flex gap-0.5 justify-start'>{children}</div>;

  return (
    <div className={twMerge('flex gap-0.5 justify-between', className)}>
      {children}
      <div className={twMerge('flex gap-0.5 overflow-x-auto', type === 'minimal' && 'hidden')} ref={divRef}>
        { editable &&
      <>
        <HistoryToolbarActions />
        <VerticalSeparator />
        <HeadingToolbarActions />
        <VerticalSeparator />
        <TextFormatToolbarActions />
        <VerticalSeparator />
        <CodeFormatToolbarActions />
      </>
        }
      </div>
      <div>
        <MarkdownToolbarActions />
      </div>
    </div>

  );
}