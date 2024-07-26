import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from '@/components/ui/context-menu';
import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { useCreateThread } from '@/lib/assistant/use-create-thread';
import { useAvailableExtensions } from '@/lib/extension/use-available-extensions';

import { PropsWithChildren } from 'react';
import { useLocation } from 'wouter';


interface Props extends PropsWithChildren { }

export function NewThreadContextMenu({ children }: Props) {
  const createThread = useCreateThread({ redirect: true });


  return (
    <ContextMenu>
      <ContextMenuTrigger>
        { children }
      </ContextMenuTrigger>
      <ContextMenuContent className="w-64">
        <ContextMenuItem inset onSelect={() => createThread()}>
          New
        </ContextMenuItem>
        <ExtensionSection />

      </ContextMenuContent>
    </ContextMenu>
  );
}

function ExtensionSection() {
  const { data, isLoading, error } = useAvailableExtensions();
  const [_, setLocation] = useLocation();
  const createThread = useCreateThread({ redirect: true });

  return (
    <>
      <ContextMenuSeparator />
      <ContextMenuItem
        className='font-bold'
        inset
        onSelect={() => setLocation('/extensions')}
        title='Manage Extensions'
      >
        Extensions
      </ContextMenuItem>
      { error && <ErrorDetails error={error} /> }
      { isLoading && <ThreeDotsLoading /> }
      { data && data.length === 0 && <ContextMenuItem inset disabled>No extensions installed</ContextMenuItem> }
      { data && data.map((extension) => (
        <ContextMenuItem
          key={extension.name}
          inset
          onSelect={() => createThread({ assistantId: extension.name })}
        >
          { extension.name }
        </ContextMenuItem>
      )) }
    </>
  );
}