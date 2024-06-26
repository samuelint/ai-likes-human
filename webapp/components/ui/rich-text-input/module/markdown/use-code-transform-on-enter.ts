import { $createCodeNode, CodeNode } from '@lexical/code';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $setBlocksType } from '@lexical/selection';
import { $getSelection, $isRangeSelection, $isTextNode, BaseSelection, COMMAND_PRIORITY_CRITICAL, KEY_DOWN_COMMAND, TextNode } from 'lexical';
import { useEffect } from 'react';
import { CODE } from './code.transformer';


export function useCodeTransformOnEnter() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand<KeyboardEvent | null>(
      KEY_DOWN_COMMAND,
      (event) => {
        const textValue = (event?.target as HTMLDivElement)?.textContent;

        if (textValue?.match(CODE.regExp) && event?.key === 'Enter') {
          event.preventDefault();

          const selection = $getSelection();
          const node = getCodeCompatibleNode(selection);
          if (selection && node && doesNodeIncludeBackticks(node)) {
            transformToCodeBlock(selection, node);
          }
        }

        return false;
      },
      COMMAND_PRIORITY_CRITICAL,
    );
  }, [editor]);
}

function transformToCodeBlock(selection: BaseSelection, node: TextNode) {
  const text = node.getTextContent();
  node.setTextContent('');
  $setBlocksType(selection, () => createCodeNodeFromText(text));
}

function doesNodeIncludeBackticks(node: TextNode): boolean {
  const text = node.getTextContent();

  return text.includes('```');
}

function getCodeCompatibleNode(selection: BaseSelection | null): TextNode | null {
  if (!selection || !$isRangeSelection(selection)) {
    return null;
  }

  const nodes = selection.getNodes();
  const node = nodes?.[0];
  if (node && $isTextNode(node)) {
    return node;
  }

  return null;
}

function createCodeNodeFromText(textValue: string): CodeNode {
  const lang = textValue.replace('```', '');
  const codeNode = $createCodeNode();
  if (lang) codeNode.setLanguage(lang);
  return codeNode;
}