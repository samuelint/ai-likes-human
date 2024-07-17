import { Alert, AlertTitle } from '@/components/ui/alert';
import { useServerStatus } from '@/lib/local-server-context';
import { AlertCircle, CircleCheck } from 'lucide-react';


export default function ServerStatus() {
  const { isUp } = useServerStatus();

  return (
    <Alert variant={!isUp ? 'destructive' : 'success'}>
      { !isUp ? <AlertCircle className="h-4 w-4" /> : <CircleCheck className="h-4 w-4" /> }
      <AlertTitle>Local Server Status</AlertTitle>
    </Alert>
  );
}
