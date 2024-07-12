import { $createParagraphNode, $getSelection, $isRangeSelection, LexicalEditor, RangeSelection } from 'lexical';
import { mutateHeading } from './heading.command';
import { when } from 'jest-when';
import { $setBlocksType } from '@lexical/selection';
import { $createHeadingNode, HeadingNode } from '@lexical/rich-text';
import { $findSelectionNodeUpToRoot } from '../_utils/contains-node';


vi.mock('lexical');
vi.mock('@lexical/selection');
vi.mock('@lexical/rich-text');
vi.mock('../_utils/contains-node');
describe('mutateHeading', () => {
  const editor = {} as LexicalEditor;
  const getNodes = vi.fn();

  const selection = {
    getNodes,
  } as unknown as RangeSelection;

  beforeEach(() => {
    vi.resetAllMocks();
    when($getSelection).mockReturnValue(selection);
  });

  it(`given no range selection,
      when mutating heading,
      then no change are made`, () => {
    when($isRangeSelection).calledWith(selection).mockReturnValue(false);

    mutateHeading('h1', editor);

    expect($setBlocksType).not.toHaveBeenCalled();
  });

  it(`given range selection,
      when mutating heading,
      then heading node is set`, () => {
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when(getNodes).mockReturnValue([]);

    mutateHeading('h1', editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, expect.anything());
    ($setBlocksType as vi.Mock).mock.lastCall[1]();
    expect($createHeadingNode).toHaveBeenCalledWith('h1');
  });

  it(`given range selection,
        and selection node already a heading node
      when mutating heading with same tag,
      then a paragraph node is set`, () => {
    const headingNode = { getTag: () => 'h1' } as HeadingNode;
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when($findSelectionNodeUpToRoot).calledWith(selection, HeadingNode).mockReturnValue(headingNode);

    mutateHeading('h1', editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, expect.anything());
    ($setBlocksType as vi.Mock).mock.lastCall[1]();
    expect($createParagraphNode).toHaveBeenCalled();
  });

  it(`given range selection,
        and selection node already a heading node
      when mutating heading with different tag,
      then a header with new tag is set`, () => {
    const headingNode = { getTag: () => 'h2' } as HeadingNode;
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when($findSelectionNodeUpToRoot).calledWith(selection, HeadingNode).mockReturnValue(headingNode);

    mutateHeading('h3', editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, expect.anything());
    ($setBlocksType as vi.Mock).mock.lastCall[1]();
    expect($createHeadingNode).toHaveBeenCalledWith('h3');
  });

  it(`given range selection,
        and selection node already a heading node
      when mutating heading with paragraph tag,
      then a paragraph node is set`, () => {
    const headingNode = { getTag: () => 'h1' } as HeadingNode;
    when($isRangeSelection).calledWith(selection).mockReturnValue(true);
    when($findSelectionNodeUpToRoot).calledWith(selection, HeadingNode).mockReturnValue(headingNode);

    mutateHeading('p', editor);

    expect($setBlocksType).toHaveBeenCalledWith(selection, expect.anything());
    ($setBlocksType as vi.Mock).mock.lastCall[1]();
    expect($createParagraphNode).toHaveBeenCalled();
  });
});