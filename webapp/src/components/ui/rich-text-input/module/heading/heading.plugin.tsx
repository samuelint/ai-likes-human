import { COMMAND_PRIORITY_NORMAL } from 'lexical';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { HeadingTagType, HeadingNode } from '@lexical/rich-text';
import { FORMAT_HEADING_COMMAND, mutateHeading } from './heading.command';


const HeadingNodes = [HeadingNode];
export { HeadingNodes };

export function HeadingPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand<HeadingTagType>(
      FORMAT_HEADING_COMMAND,
      mutateHeading,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  return null;
}
