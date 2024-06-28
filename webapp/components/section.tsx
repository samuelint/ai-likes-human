import { ReactNode } from 'react';
import { H2 } from './h2';


interface Props {
  title: ReactNode
  children?: ReactNode
}

export function Section({ title, children }: Props) {
  return (
    <section className='pl-4'>
      <H2 className='text-xl'>{title}</H2>
      { children }
    </section>
  );
}
