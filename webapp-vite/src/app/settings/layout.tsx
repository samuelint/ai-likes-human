import { ReactNode } from 'react';


interface Props {
  children: ReactNode
}
export default function Layout({ children }: Props) {
  return <div className="w-full h-full flex flex-col justify-between p-6 overflow-y-auto">{children}</div>;
}
