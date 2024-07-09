import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { ChatMessageProps, Message } from './message';
import { MessageRunDetails } from './message-run-details';


interface Props extends Omit<ChatMessageProps, 'type' | 'className' | 'actions'> {
  run?: Run | null
}

export function AIChatMessage({ run, children, ...props }: Props) {
  return <Message
    type='ai'
    className='w-max max-w-[75%]'
    {...props}
  >
    {run && <MessageRunDetails run={run} />}
    {children}
  </Message>;
}