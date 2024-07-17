import { PropsWithChildren } from 'react';
import { ServerStatusProvider } from './lib/local-server-context';

export default function Providers({ children }: PropsWithChildren) {
  return (
    <ServerStatusProvider>
      {children }
    </ServerStatusProvider>
  );
}
