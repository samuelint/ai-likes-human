import { cn } from '@/lib/utils';
import { ReactNode } from 'react';


interface Props {
  children: ReactNode
  className?: string
}

export function H2({ children, className }: Props) {
  return (
    <h2 className={cn('text-2xl font-bold pb-4', className)}>{children}</h2>
  );
}

