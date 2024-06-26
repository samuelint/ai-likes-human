import { $createHeadingNode, HeadingNode, HeadingTagType as LibHeadingTagType } from '@lexical/rich-text';
import { $setBlocksType } from '@lexical/selection';
import { $createParagraphNode, $getSelection, $isRangeSelection, CommandListener, createCommand } from 'lexical';
import { $findSelectionNodeUpToRoot } from '../_utils/contains-node';


export type HeadingTagType = `${LibHeadingTagType | 'p'}`;
export const FORMAT_HEADING_COMMAND = createCommand<HeadingTagType>('FORMAT_HEADING_COMMAND');

export const mutateHeading: CommandListener<HeadingTagType> = (tag) => {
  const selection = $getSelection();
  if ($isRangeSelection(selection)) {
    const headingNode = $findSelectionNodeUpToRoot(selection, HeadingNode);
    const currentHeadingType = headingNode?.getTag();

    $setBlocksType(selection, () => currentHeadingType === tag || tag === 'p' ? $createParagraphNode() : $createHeadingNode(tag));
  }

  return true;
};
