import { H1 } from '@/components/h1';
import { PropsWithChildren, ReactNode } from 'react';

interface Props extends PropsWithChildren {
  title: ReactNode
}

export function PageSection({ title, children }: Props) {
  return (
    <div className="w-full h-full flex flex-col justify-between p-6 overflow-y-auto">
      <H1>{title}</H1>
      {children}
    </div>
  );
}
