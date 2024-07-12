import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { CodeHighlightNode, CodeNode, registerCodeHighlighting } from '@lexical/code';
import { FORMAT_CODE_COMMAND, formatCode } from './format-code.command';
import { COMMAND_PRIORITY_NORMAL } from 'lexical';


const CodeHighlightNodes = [
  CodeHighlightNode,
  CodeNode,
];
export { CodeHighlightNodes };


export function CodeHighlightPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return registerCodeHighlighting(editor);
  }, [editor]);

  useEffect(() => {
    return editor.registerCommand(
      FORMAT_CODE_COMMAND,
      formatCode,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  return (<></>);
}
