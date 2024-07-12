import { CodeBracketIcon } from '@heroicons/react/16/solid';
import { ToolbarAction } from '../../toolbar';
import { FORMAT_CODE_COMMAND } from './format-code.command';


export function CodeBlockAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_CODE_COMMAND,
        payload: undefined
      }}
      title='Code'
    >
      <CodeBracketIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function CodeFormatToolbarActions() {
  return (
    <>
      <CodeBlockAction />
    </>
  );
}
