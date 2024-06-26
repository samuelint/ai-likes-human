import { MarkdownShortcutPlugin } from '@lexical/react/LexicalMarkdownShortcutPlugin';
import { HeadingNode, QuoteNode } from '@lexical/rich-text';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { APPLY_MARKDOWN_COMMAND, ToggleMarkdownType, toggleMarkdown } from './markdown.command';
import { COMMAND_PRIORITY_NORMAL } from 'lexical';
import { useEffect } from 'react';
import { _TRANSFORMERS } from './transformers';
import { HorizontalRuleNode } from '@lexical/react/LexicalHorizontalRuleNode';
import { HorizontalRulePlugin } from '@lexical/react/LexicalHorizontalRulePlugin';
import { useCodeTransformOnEnter } from './use-code-transform-on-enter';


const MarkdownNodes = [
  HeadingNode,
  QuoteNode,
  HorizontalRuleNode,
];
export { MarkdownNodes };

export function MarkdownPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand<ToggleMarkdownType>(
      APPLY_MARKDOWN_COMMAND,
      toggleMarkdown,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  useCodeTransformOnEnter();

  return <>
    <MarkdownShortcutPlugin transformers={_TRANSFORMERS} />
    <HorizontalRulePlugin />
  </>;
}
