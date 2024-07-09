import { ReactNode } from 'react';
import { ChatMessageDto, isAiMessage } from './chat.type';
import NewMessage, { ChatNewMessageProps } from './new-message';
import { ThreeDotsLoading } from './ui/loading';
import { Message, ChatMessageProps } from './message';
import { AIChatMessage } from './chat-ai-message';


interface Props extends ChatNewMessageProps {
  messages: ChatMessageDto[]
  isLoading?: boolean
  children?: ReactNode
  details?: ReactNode
}

export default function Chat({ messages, isLoading, children, details, ...props }: Props) {
  return (
    <div role='presentation' className='h-full flex flex-col'>
      <div className="flex flex-col h-full overflow-y-auto py-4 px-12 sm:px-28 md:px-32 xl:px-80">
        {children}
        <div className="space-y-4">
          { messages.map((message) => {
            if (isAiMessage(message)) {
              return <AIChatMessage key={message.id} content={message.content}/>;
            } else {
              return <UserChatMessage key={message.id} content={message.content}/>;
            }
          })
          }
          { isLoading && <ThreeDotsLoading className="bg-blue-900 dark:bg-blue-50" /> }
        </div>
      </div>
      <div className='mt-auto py-4 px-12 sm:px-28 md:px-32 xl:px-80'>
        { details && <div className='flex w-full justify-end'>{details}</div> }
        <NewMessage {...props} />
      </div>
    </div>
  );
}


function UserChatMessage(props: Omit<ChatMessageProps, 'type' | 'className'>) {
  return <Message type='user' className='w-max max-w-[75%] ml-auto' {...props} />;
}
