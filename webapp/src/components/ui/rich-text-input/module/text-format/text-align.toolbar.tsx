import { Bars3BottomLeftIcon, Bars3BottomRightIcon, Bars3Icon } from '@heroicons/react/16/solid';
import { ToolbarAction } from '../../toolbar';
import { FORMAT_ELEMENT_COMMAND } from 'lexical';


export function AlignLeftAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_ELEMENT_COMMAND,
        payload: 'left'
      }}
    >
      <Bars3BottomLeftIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function AlignCenterAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_ELEMENT_COMMAND,
        payload: 'center'
      }}
    >
      <Bars3Icon className='w-4 h-4'/>
    </ToolbarAction>
  );
}


export function AlignRightAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_ELEMENT_COMMAND,
        payload: 'right'
      }}
    >
      <Bars3BottomRightIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function TextAlignToolbarActions() {
  return (
    <>
      <AlignLeftAction />
      <AlignCenterAction />
      <AlignRightAction />
    </>
  );
}
