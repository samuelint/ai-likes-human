import { $setBlocksType } from '@lexical/selection';
import { $createParagraphNode, $getSelection, $isRangeSelection, CommandListener, createCommand } from 'lexical';
import { $createBannerNode, BannerNode } from './banner.node';
import { $findSelectionNodeUpToRoot } from '../_utils/contains-node';


export const INSERT_BANNER_COMMAND = createCommand('INSERT_BANNER');


export const insertBannerCommand: CommandListener<undefined> = () => {
  const selection = $getSelection();
  if ($isRangeSelection(selection)) {
    const bannerNode = $findSelectionNodeUpToRoot(selection, BannerNode);

    $setBlocksType(selection, bannerNode ? $createParagraphNode : $createBannerNode);
  }

  return true;
};

