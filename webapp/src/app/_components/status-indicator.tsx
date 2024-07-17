import { Link } from 'wouter';
import { TriangleAlert } from 'lucide-react';
import { useServerStatus } from '@/lib/local-server-context';


export default function StatusIndicator() {
  const { isUp } = useServerStatus();

  if (!isUp) {
    return (
      <Link href="/settings" title='Server is not reachable'>
        <TriangleAlert className='w-4 h-4 text-orange-500'/>
      </Link>
    );
  }

}
