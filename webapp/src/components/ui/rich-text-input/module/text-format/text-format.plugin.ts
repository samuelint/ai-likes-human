import { COMMAND_PRIORITY_NORMAL } from 'lexical';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { FORMAT_QUOTE_COMMAND, formatQuote } from './format-quote.command';
import { QuoteNode } from '@lexical/rich-text';


const TextFormatNodes = [QuoteNode];
export { TextFormatNodes };

export function TextFormatPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand(
      FORMAT_QUOTE_COMMAND,
      formatQuote,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  return null;
}
