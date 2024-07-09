'use client';
import Chat from '@/components/chat';
import { useCreateThread } from '@/lib/use-create-thread';
import { useCurrentModel } from '@/lib/use-current-model';
import { useState } from 'react';


export default function NewThreadPrompt() {
  const { data: model } = useCurrentModel();
  const [input, setInput] = useState('');
  const createThread = useCreateThread({ redirect: true });

  return (
    <Chat
      messages={[]}
      input={input}
      onChange={(event) => setInput(event.target.value)}
      onSubmit={(event) => {
        event.preventDefault();
        event.stopPropagation();
        createThread(input);
      }}
      details={<span className='text-slate-400 text-xs'>{model}</span>}
    />
  );
}
