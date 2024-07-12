import { CodeBracketIcon, ChatBubbleBottomCenterIcon } from '@heroicons/react/16/solid';
import { ToolbarAction } from '../../toolbar';
import { FORMAT_TEXT_COMMAND } from 'lexical';
import { FORMAT_QUOTE_COMMAND } from './format-quote.command';
import { IS_APPLE } from '../../utils/platform';


export function BoldAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_TEXT_COMMAND,
        payload: 'bold'
      }}
      title={IS_APPLE ? 'Bold (⌘B)' : 'Bold (Ctrl+B)'}
    >
    B
    </ToolbarAction>
  );
}

export function ItalicAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_TEXT_COMMAND,
        payload: 'italic'
      }}
      title={IS_APPLE ? 'Italic (⌘I)' : 'Italic (Ctrl+I)'}
    >
    I
    </ToolbarAction>
  );
}

export function UnderlineAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_TEXT_COMMAND,
        payload: 'underline'
      }}
      title={IS_APPLE ? 'Underline (⌘U)' : 'Underline (Ctrl+U)'}
    >
    U
    </ToolbarAction>
  );
}

export function CodeAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_TEXT_COMMAND,
        payload: 'code'
      }}
      title='Code'
    >
      <CodeBracketIcon className='w-2 h-2'/>
    </ToolbarAction>
  );
}

export function QuoteBlockAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_QUOTE_COMMAND,
        payload: undefined
      }}
      title='Quote'
    >
      <ChatBubbleBottomCenterIcon className='w-4 h-4'/>
    </ToolbarAction>
  );
}

export function TextFormatToolbarActions() {
  return (
    <>
      <BoldAction />
      <ItalicAction />
      <UnderlineAction />
      <QuoteBlockAction />
    </>
  );
}
