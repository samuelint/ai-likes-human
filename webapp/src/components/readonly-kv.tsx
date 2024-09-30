import { ReactNode } from 'react';


interface Props {
  name: string
  children?: ReactNode
}

export function ReadonlyKV({ name, children }: Props) {
  return (
    <div className='flex text-gray-500 text-sm items-baseline'>
      <span className='pr-2 align-baseline'>{name}: </span><span className='text-xs font-mono align-baseline'>{children}</span>
    </div>
  );
}
