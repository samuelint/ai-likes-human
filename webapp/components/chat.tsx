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
}

export default function Chat({ messages, isLoading, children, ...props }: Props) {
  return (
    <>
      <div className="flex-1 overflow-y-auto p-6">
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
      <NewMessage {...props} />
    </>
  );
}


function UserChatMessage(props: Omit<ChatMessageProps, 'type' | 'className'>) {
  return <Message type='user' className='w-max max-w-[75%] ml-auto' {...props} />;
}
