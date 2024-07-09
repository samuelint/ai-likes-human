import { ReactNode } from 'react';
import { ChatMessageDto, isAiMessage } from './chat.type';
import NewMessage, { ChatNewMessageProps } from './new-message';
import { ThreeDotsLoading } from './ui/loading';
import { Message, ChatMessageProps } from './message';
import { AIChatMessage } from './chat-ai-message';
import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { Dictionary } from 'lodash';


interface Props extends ChatNewMessageProps {
  messages: ChatMessageDto[]
  byIdRuns?: Dictionary<Run>
  isLoading?: boolean
  children?: ReactNode
  details?: ReactNode
}

export default function Chat({ messages, byIdRuns = {}, isLoading, children, details, ...props }: Props) {

  return (
    <div role='presentation' className='h-full flex flex-col'>
      <div className="flex flex-col h-full overflow-y-auto py-4 px-12 sm:px-28 md:px-32 xl:px-80">
        {children}
        <div className="space-y-4">
          { messages.map((message) => {
            const run = message.run_id ? byIdRuns[message.run_id] : null;
            if (isAiMessage(message)) {
              return <AIChatMessage key={message.id} content={message.content} run={run}/>;
            } else {
              return <UserChatMessage key={message.id} content={message.content}/>;
            }
          })
          }
          { isLoading && <ThreeDotsLoading className="bg-blue-900 dark:bg-blue-50" /> }
        </div>
      </div>
      <div className='mt-auto py-4 px-12 sm:px-28 md:px-32 xl:px-80'>
        <NewMessage {...props}>
          { details && <div className='flex w-full justify-end'>{details}</div> }
        </NewMessage>
      </div>
    </div>
  );
}


function UserChatMessage(props: Omit<ChatMessageProps, 'type' | 'className'>) {
  return <Message type='user' className='w-max max-w-[75%] ml-auto' {...props} />;
}
