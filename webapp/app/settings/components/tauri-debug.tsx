'use client';
import { useEffect, useState } from 'react';


import { attachConsole } from 'tauri-plugin-log-api';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@/components/ui/button';


export default function TauriDebug() {
  const [events, setEvents] = useState<string[]>([]);

  const addEvent = (event: string) => {
    setEvents((prev) => [...prev, event]);
  };

  useEffect(() => {
    attachConsole();
  });

  return (
    <section className='w-full flex flex-col gap-4'>
      <div className="w-full flex flex-row justify-evenly gap-2">
        <Button onClick={() => {
          invoke('start_server').then((r) => addEvent(`Start ${JSON.stringify(r)}`)).catch((e) => addEvent(`Error ${JSON.stringify(e)}`));
        }}>Start Server</Button>
        <Button onClick={() => {
          invoke('stop_server').then((r) => addEvent(`Stop ${JSON.stringify(r)}`)).catch((e) => addEvent(`Error ${JSON.stringify(e)}`));
        }}>Stop Server</Button>
      </div>
      <ul className='text-red-500 flex flex-col gap-2'>
        {events.map((e, i) => <li key={i}>{e}</li>)}
      </ul>
    </section>

  );
}
