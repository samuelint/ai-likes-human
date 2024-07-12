
import { useEffect } from 'react';


import { attachConsole } from 'tauri-plugin-log-api';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { useServerStatus } from '@/lib/use-server-status';
import { ThreeDotsLoading } from '@/components/ui/loading';


export default function LocalServerSettings() {
  const { toast } = useToast();
  const { data, isLoading } = useServerStatus();

  useEffect(() => {
    attachConsole();
  });

  if (isLoading) {
    return <ThreeDotsLoading />;
  }
  if (data) {
    return (
      <Button
        onClick={() => {
          invoke('start_server')
            .then((r) => toast({
              title: 'Server started',
              description: JSON.stringify(r),
            }))
            .catch((e) => toast({
              title: 'Server start error',
              description: e.message,
              variant: 'destructive',
            }));
        }}
      >Start Server</Button>);
  }
  else {
    return (<Button
      onClick={() => {
        invoke('stop_server')
          .then((r) => toast({
            title: 'Server stopped',
            description: JSON.stringify(r),
          }))
          .catch((e) => toast({
            title: 'Server stop error',
            description: e.message,
            variant: 'destructive',
          }));
      }}>Stop Server</Button>);
  }
}
