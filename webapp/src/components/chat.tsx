import { ReactNode } from 'react';
import { ChatMessageDto } from './chat.type';
import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { Dictionary } from 'lodash';
import MessageCollection from './message-collection';


interface Props {
  messages?: ChatMessageDto[]
  byIdRuns?: Dictionary<Run>
  isLoading?: boolean
  children?: ReactNode
}

export default function Chat({ messages = [], byIdRuns = {}, isLoading, children }: Props) {

  return (
    <div role='presentation' className='h-full flex flex-col overflow-auto'>
      <MessageCollection messages={messages} byIdRuns={byIdRuns} isLoading={isLoading}/>
      <div className='relative mt-auto py-2 px-2 md:px-20 lg:px-32 w-full'>
        { children }
      </div>
    </div>
  );
}
