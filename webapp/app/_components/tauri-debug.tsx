'use client';
import { useEffect, useState } from 'react';


import { attachConsole } from 'tauri-plugin-log-api';
import { invoke } from '@tauri-apps/api/tauri';


export default function TauriDebug() {
  const [events, setEvents] = useState<string[]>([]);

  const addEvent = (event: string) => {
    setEvents((prev) => [...prev, event]);
  };

  useEffect(() => {
    attachConsole();
  });

  return (
    <div className="flex flex-col gap-2">
      <button onClick={() => {
        invoke('start_server').then((r) => addEvent(`Start ${JSON.stringify(r)}`)).catch((e) => addEvent(`Error ${JSON.stringify(e)}`));
      }}>Start Server</button>
      <button onClick={() => {
        invoke('stop_server').then((r) => addEvent(`Stop ${JSON.stringify(r)}`)).catch((e) => addEvent(`Error ${JSON.stringify(e)}`));
      }}>Stop Server</button>
      <div className='text-red-500 flex gap-2'>
        {events.map((e, i) => <span key={i}>{e}</span>)}
      </div>
    </div>
  );
}
