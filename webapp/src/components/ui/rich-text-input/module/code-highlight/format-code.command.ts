import { $createCodeNode } from '@lexical/code';
import { $setBlocksType } from '@lexical/selection';
import { $getSelection, $isRangeSelection, CommandListener, createCommand } from 'lexical';


export const FORMAT_CODE_COMMAND = createCommand<undefined>('FORMAT_CODE');

export const formatCode: CommandListener<undefined> = () => {
  let selection = $getSelection();

  if (selection !== null) {
    if (selection.isCollapsed()) {
      $setBlocksType(selection, () => $createCodeNode());
    } else {
      const textContent = selection.getTextContent();
      const codeNode = $createCodeNode();
      selection.insertNodes([codeNode]);
      selection = $getSelection();
      if ($isRangeSelection(selection))
        selection.insertRawText(textContent);
    }
  }

  return true;
};
