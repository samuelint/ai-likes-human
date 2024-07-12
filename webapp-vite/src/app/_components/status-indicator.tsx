
import { useServerStatus } from '@/lib/use-server-status';
import { TriangleAlert } from 'lucide-react';


export default function StatusIndicator() {
  const { data, error } = useServerStatus();

  if (error || data?.status !== 'ok') {
    return (
      <a href="/settings" title='Server is not reachable'>
        <TriangleAlert className='w-4 h-4 text-orange-500'/>
      </a>
    );
  }

}
