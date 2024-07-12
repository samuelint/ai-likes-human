import { $createParagraphNode, $getSelection, $isRangeSelection, LexicalEditor, RangeSelection } from 'lexical';
import { insertBannerCommand } from './banner.command';
import { $setBlocksType } from '@lexical/selection';
import { $createBannerNode, BannerNode } from './banner.node';
import { when } from 'jest-when';
import { $findSelectionNodeUpToRoot } from '../_utils/contains-node';


vi.mock('lexical');
vi.mock('@lexical/selection');
vi.mock('./banner.node');
vi.mock('../_utils/contains-node');
describe('insertBannerCommand', () => {
  const editor = {} as LexicalEditor;
  const selection = {
  } as unknown as RangeSelection;

  beforeEach(() => {
    when($getSelection).mockReturnValue(selection);
  });

  it(`given no existing banner node in selection,
      when inserting banner,
      then banner is inserted`, () => {
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when($findSelectionNodeUpToRoot).calledWith(selection, BannerNode).mockReturnValue(undefined);

    insertBannerCommand(undefined, editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, $createBannerNode);
  });

  it(`given existing banner node in selection,
      when inserting banner,
      then banner is removed`, () => {
    const bannerNode = Object.create(BannerNode.prototype);
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when($findSelectionNodeUpToRoot).calledWith(selection, BannerNode).mockReturnValue(bannerNode);

    insertBannerCommand(undefined, editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, $createParagraphNode);
  });
});