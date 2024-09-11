import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { stopLocalServer } from '@/lib/tauri-command/server-status';


export default function StopLocalServer() {
  const { toast } = useToast();

  return (
    <Button
      onClick={() => {
        stopLocalServer()
          .then((r) => toast({
            title: 'Server stopped',
            description: JSON.stringify(r),
          }))
          .catch((e) => toast({
            title: 'Server stop error',
            description: e.message,
            variant: 'destructive',
          }));
      }}>Stop Server</Button>
  );
}
