import { PropsWithChildren, useEffect, useState } from 'react';
import { ToolbarButton } from './toolbar-button';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { COMMAND_PRIORITY_CRITICAL, CommandPayloadType, LexicalCommand } from 'lexical';
import { mergeRegister } from '@lexical/utils';


interface ToolbarAction<TCommand extends LexicalCommand<unknown>> {
  command: TCommand
  payload: CommandPayloadType<TCommand>
}

interface Props<TCommand extends LexicalCommand<unknown>, TCanDoCommand extends LexicalCommand<unknown> = LexicalCommand<unknown>> extends PropsWithChildren {
  action: ToolbarAction<TCommand>
  canDoCommand?: TCanDoCommand
  title?: string
}

export function ToolbarAction<TCommand extends LexicalCommand<unknown>, TCanDoCommand extends LexicalCommand<boolean> = LexicalCommand<boolean>>({ action, canDoCommand, title, children }: Props<TCommand, TCanDoCommand>) {
  const [editor] = useLexicalComposerContext();
  const [canDoAction, setCanDoAction] = useState(canDoCommand ? false : true);

  useEffect(() => {
    if (canDoCommand) {
      return mergeRegister(
        editor.registerCommand<boolean>(
          canDoCommand,
          (payload) => {
            setCanDoAction(payload);
            return false;
          },
          COMMAND_PRIORITY_CRITICAL,
        ),
      );
    }
  }, [canDoCommand, editor]);

  return (
    <ToolbarButton title={title} disabled={!canDoAction} onClick={() => editor.dispatchCommand(action.command, action.payload)}>{ children }</ToolbarButton>
  );
}