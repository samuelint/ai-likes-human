import { ReactNode } from 'react';
import { ChatMessageDto, isAiMessage } from './chat.type';
import { ThreeDotsLoading } from './ui/loading';
import { Message, ChatMessageProps } from './message';
import { AIChatMessage } from './chat-ai-message';
import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { Dictionary } from 'lodash';


interface Props {
  messages?: ChatMessageDto[]
  byIdRuns?: Dictionary<Run>
  isLoading?: boolean
  children?: ReactNode
}

export default function Chat({ messages = [], byIdRuns = {}, isLoading, children }: Props) {

  return (
    <div role='presentation' className='h-full flex flex-col overflow-auto'>
      <div className="flex flex-col h-full overflow-y-auto py-4 px-2 md:px-20 lg:px-32">
        <div className="space-y-6">
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
      <div className='mt-auto py-4 px-2 md:px-20 lg:px-32'>
        { children }
      </div>
    </div>
  );
}


function UserChatMessage(props: Omit<ChatMessageProps, 'type' | 'className'>) {
  return <Message type='user' className='w-max max-w-[75%] ml-auto' {...props} />;
}
