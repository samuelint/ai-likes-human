import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from '@/components/ui/context-menu';
import { useCreateThread } from '@/lib/assistant/use-create-thread';

import { PropsWithChildren } from 'react';


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
      </ContextMenuContent>
    </ContextMenu>
  );
}
