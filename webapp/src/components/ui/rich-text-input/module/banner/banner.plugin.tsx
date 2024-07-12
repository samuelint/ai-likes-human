import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { COMMAND_PRIORITY_NORMAL } from 'lexical';
import { useEffect } from 'react';
import { BannerNode } from './banner.node';
import { INSERT_BANNER_COMMAND, insertBannerCommand } from './banner.command';


const BannerNodes = [
  BannerNode,
];
export { BannerNodes };

export function BannerPlugin() {
  const [editor] = useLexicalComposerContext();

  if (!editor.hasNode(BannerNode)) {
    throw new Error(`BannerPlugin: '${BannerNode.name}' not registered`);
  }

  useEffect(() => {
    return editor.registerCommand(
      INSERT_BANNER_COMMAND,
      insertBannerCommand,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  return null;
}
