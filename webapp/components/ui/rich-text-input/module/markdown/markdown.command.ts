import { $createCodeNode, $isCodeNode, CodeNode } from '@lexical/code';
import { $convertFromMarkdownString, $convertToMarkdownString } from '@lexical/markdown';
import { $createTextNode, $getRoot, CommandListener, RootNode, createCommand } from 'lexical';
import { _TRANSFORMERS } from './transformers';


export type ToggleMarkdownType = undefined;
export const APPLY_MARKDOWN_COMMAND = createCommand<ToggleMarkdownType>('APPLY_MARKDOWN_COMMAND');

export const toggleMarkdown: CommandListener<ToggleMarkdownType> = () => {
  const root = $getRoot();
  const markdownNode = getMarkdownNode(root);

  if (markdownNode) {
    markdownToNode(markdownNode);
  } else {
    nodeToMarkdown(root);
  }

  return true;
};

function getMarkdownNode(root: RootNode): CodeNode | null {
  const firstChild = root.getFirstChild();
  if ($isCodeNode(firstChild) && firstChild.getLanguage() === 'markdown') {
    return firstChild;
  }

  return null;
}

function markdownToNode(node: CodeNode) {
  $convertFromMarkdownString(node.getTextContent(), _TRANSFORMERS);
}

function nodeToMarkdown(root: RootNode) {
  const markdown = $convertToMarkdownString(_TRANSFORMERS);
  root.clear().append($createCodeNode('markdown').append($createTextNode(markdown)));
}
