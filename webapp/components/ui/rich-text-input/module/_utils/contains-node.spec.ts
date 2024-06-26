import { ElementNode, LexicalNode, RangeSelection } from 'lexical';
import { $findSelectionNodeUpToRoot } from './contains-node';
import { when } from 'jest-when';


class NodeFixture extends ElementNode {
  public getParents() {
    return [];
  }
}

describe('$findSelectionNodeUpToRoot', () => {
  const getNodes = jest.fn();
  const selection = {
    getNodes,
  } as unknown as RangeSelection;

  beforeEach(() => {
    jest.resetAllMocks();
  });

  it(`given range selection without nodes and any class ctor,
      when finding up to root,
      then no node is returned`, () => {
    when(getNodes).mockReturnValue([]);

    const result = $findSelectionNodeUpToRoot(selection, NodeFixture);

    expect(result).toBeUndefined();
  });

  it(`given range selection with nodes containing instance of node,
      when finding up to root,
      then instance is returned`, () => {
    const node = Object.create(NodeFixture.prototype);
    when(getNodes).mockReturnValue([node]);

    const result = $findSelectionNodeUpToRoot(selection, NodeFixture);

    expect(result).toBe(node);
  });

  it(`given range selection with nodes parent containing instance of node,
      when finding up to root,
      then instance is returned`, () => {
    const node = Object.create(NodeFixture.prototype);
    const parentNode = { getParents: () => [node] } as unknown as LexicalNode;
    when(getNodes).mockReturnValue([parentNode]);

    const result = $findSelectionNodeUpToRoot(selection, NodeFixture);

    expect(result).toBe(node);
  });
});