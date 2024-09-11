import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { startLocalServer } from '@/lib/tauri-command/server-status';


export default function StartLocalServer() {
  const { toast } = useToast();

  return (
    <Button
      onClick={() => {
        startLocalServer()
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
    >Start Server</Button>
  );
}
