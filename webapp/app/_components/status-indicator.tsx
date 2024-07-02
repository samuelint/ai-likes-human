'use client';
import { useServerStatus } from '@/lib/use-server-status';
import { TriangleAlert } from 'lucide-react';
import Link from 'next/link';


export default function StatusIndicator() {
  const { data, error } = useServerStatus();

  if (error || data?.status !== 'ok') {
    return (
      <Link href="/settings" title='Server is not reachable'>
        <TriangleAlert className='w-4 h-4 text-orange-500'/>
      </Link>
    );
  }

}
