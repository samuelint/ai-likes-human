import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from '@/components/ui/context-menu';

import { PropsWithChildren } from 'react';


interface Props extends PropsWithChildren {
  onDelete?: () => void
}

export function ThreadPreviewContextMenu({ onDelete, children }: Props) {
  return (
    <ContextMenu>
      <ContextMenuTrigger>
        { children }
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem inset onSelect={() => onDelete && onDelete()}>
        Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}