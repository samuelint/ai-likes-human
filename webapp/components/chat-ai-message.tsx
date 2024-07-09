import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { ChatMessageProps, Message } from './message';
import { Avatar, AvatarFallback } from './ui/avatar';
import { MessageRunDetailsTooltip } from './message-run-details';



interface Props extends Omit<ChatMessageProps, 'type' | 'className' | 'actions'> {
  run?: Run | null
}

export function AIChatMessage({ run, children, ...props }: Props) {
  return (
    <div className='flex'>
      <MessageRunDetailsTooltip run={run}>
        <Avatar className='select-none'>
          <AvatarFallback>A</AvatarFallback>
        </Avatar>
      </MessageRunDetailsTooltip>
      <Message
        type='ai'
        className='w-max max-w-[75%]'
        {...props}
      >
        {children}
      </Message>
    </div>
  );
}