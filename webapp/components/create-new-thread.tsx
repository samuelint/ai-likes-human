import { Plus } from 'lucide-react';
import { Button } from './ui/button';
import { cn } from '@/lib/utils';


interface Props {
  className?: string
  onClick?: () => void
}

export function CreateNewThread({ className, onClick }: Props) {
  return (
    <Button
      variant='outline'
      onClick={onClick}
      className={cn('flex flex-col items-start', className)}
    >
      <Plus className='w-10 h-10'/>
    </Button>
  );
}
