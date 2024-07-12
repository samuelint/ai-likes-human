import { $createQuoteNode } from '@lexical/rich-text';
import { $setBlocksType } from '@lexical/selection';
import { $getSelection, CommandListener, createCommand } from 'lexical';


export const FORMAT_QUOTE_COMMAND = createCommand<undefined>('FORMAT_QUOTE');

export const formatQuote: CommandListener<undefined> = () => {
  const selection = $getSelection();
  $setBlocksType(selection, () => $createQuoteNode());

  return true;
};
