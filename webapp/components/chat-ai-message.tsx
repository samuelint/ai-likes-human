import { useCopyToClipboard } from 'usehooks-ts';
import { ChatMessageProps, Message } from './message';
import { CopyIcon } from './icon/copy.icon';


export function AIChatMessage(props: Omit<ChatMessageProps, 'type' | 'className' | 'actions'>) {
  const [, copy] = useCopyToClipboard();

  return <Message
    type='ai'
    className='w-max max-w-[75%]'
    actions={<>
      <button onClick={() => props.content && copy(props.content)}><CopyIcon className='w-4 h-4 text-slate-600' /></button>
    </>}
    {...props}
  />;
}