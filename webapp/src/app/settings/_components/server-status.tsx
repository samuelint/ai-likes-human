
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { ThreeDotsLoading } from '@/components/ui/loading';

import { useServerStatus } from '@/lib/use-server-status';
import { AlertCircle, CircleCheck } from 'lucide-react';


export default function ServerStatus() {
  const { data, error, isLoading } = useServerStatus();

  return (
    <Alert variant={error ? 'destructive' : 'success'}>
      { error ? <AlertCircle className="h-4 w-4" /> : <CircleCheck className="h-4 w-4" /> }

      <AlertTitle><b>Local</b> Server Status</AlertTitle>
      <AlertDescription>
        { isLoading && <ThreeDotsLoading /> }
        { error && error.message }
        { data && data.status === 'ok' && 'Server is reachable' }
      </AlertDescription>
    </Alert>
  );
}
