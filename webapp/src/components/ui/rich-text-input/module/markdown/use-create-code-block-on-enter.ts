import { $createCodeNode, CodeNode } from '@lexical/code';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $setBlocksType } from '@lexical/selection';
import { $getSelection, $isRangeSelection, $isTextNode, BaseSelection, COMMAND_PRIORITY_CRITICAL, KEY_ENTER_COMMAND, TextNode } from 'lexical';
import { useEffect } from 'react';


export function useCreateCodeBlockOnEnter() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand<KeyboardEvent | null>(
      KEY_ENTER_COMMAND,
      (event) => {
        const textValue = (event?.target as HTMLDivElement)?.textContent;

        if (event && textValue?.match(/```(?:\w+)?/g)) {
          event.preventDefault();

          const selection = $getSelection();
          const node = getCodeCompatibleNode(selection);
          if (selection && node && doesNodeIncludeBackticks(node)) {
            transformToCodeBlock(selection, node);
            return true;
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
  $setBlocksType(selection, () => {
    const codeNode = createCodeNodeFromText(text);

    return codeNode;
  });
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
  return $createCodeNode(lang);
}