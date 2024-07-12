import { cn } from '@/lib/utils';
import { ReactNode } from 'react';


interface Props {
  children: ReactNode
}

export function H1({ children }: Props) {
  return (
    <h1 className={cn('text-3xl font-bold pb-6')}>{children}</h1>
  );
}

