import { ArrowUturnLeftIcon, ArrowUturnRightIcon } from '@heroicons/react/16/solid';
import { CAN_REDO_COMMAND, CAN_UNDO_COMMAND, REDO_COMMAND, UNDO_COMMAND } from 'lexical';
import { ToolbarAction } from '../../toolbar';
import { IS_APPLE } from '../../utils/platform';



export function UndoAction() {
  return (
    <ToolbarAction
      action={{
        command: UNDO_COMMAND,
        payload: undefined
      }}
      canDoCommand={CAN_UNDO_COMMAND}
      title={IS_APPLE ? 'Undo (⌘Z)' : 'Undo (Ctrl+Z)'}
    >
      <ArrowUturnLeftIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function RedoAction() {
  return (
    <ToolbarAction
      action={{
        command: REDO_COMMAND,
        payload: undefined
      }}
      canDoCommand={CAN_REDO_COMMAND}
      title={IS_APPLE ? 'Redo (⌘Y)' : 'Redo (Ctrl+Y)'}
    >
      <ArrowUturnRightIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function HistoryToolbarActions() {

  return (
    <>
      <UndoAction/>
      <RedoAction/>
    </>
  );
}
