/*
 * This code is not SOLID. It's a almost copy/paste of the official example.
 * https://github.com/facebook/lexical/blob/main/packages/lexical-playground/src/plugins/CodeActionMenuPlugin/components/CopyButton/index.tsx
 */
import { $isCodeNode } from '@lexical/code';
import {
  $getNearestNodeFromDOMNode,
  $getSelection,
  $setSelection,
  LexicalEditor,
} from 'lexical';
import { DocumentDuplicateIcon } from '@heroicons/react/16/solid';


interface Props {
  editor: LexicalEditor;
  getCodeDOMNode: () => HTMLElement | null;
}

export function CopyButton({ editor, getCodeDOMNode }: Props) {
  async function handleClick(): Promise<void> {
    const codeDOMNode = getCodeDOMNode();

    if (!codeDOMNode) {
      return;
    }

    let content = '';

    editor.update(() => {
      const codeNode = $getNearestNodeFromDOMNode(codeDOMNode);

      if ($isCodeNode(codeNode)) {
        content = codeNode.getTextContent();
      }

      const selection = $getSelection();
      $setSelection(selection);
    });

    try {
      await navigator.clipboard.writeText(content);
    } catch (err) {
      console.error('Failed to copy: ', err);
    }
  }

  return (
    <button className="code-action-menu__button" onClick={handleClick} aria-label="copy">
      <DocumentDuplicateIcon className='code-action-menu__icon' />
    </button>
  );
}