import { ChatMessageProps, Message } from './message';


export function AIChatMessage(props: Omit<ChatMessageProps, 'type' | 'className' | 'actions'>) {
  return <Message
    type='ai'
    className='w-max max-w-[75%]'
    {...props}
  />;
}