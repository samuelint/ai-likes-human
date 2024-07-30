import { ReactNode } from 'react';
import { H2 } from './h2';
import { cn } from '@/lib/utils';


interface Props {
  id?: string
  title: ReactNode
  children?: ReactNode
  className?: string
}

export function Section({ id, title, className, children }: Props) {
  return (
    <section id={id} className='py-4'>
      <H2 className='text-xl'>{title}</H2>
      <div className={cn('w-full flex flex-col gap-4 px-4', className)}>
        { children }
      </div>
    </section>
  );
}
