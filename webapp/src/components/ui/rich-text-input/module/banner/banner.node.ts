import {
  $createParagraphNode,
  EditorConfig,
  ElementNode,
  LexicalEditor,
  LexicalNode,
  RangeSelection,
  SerializedElementNode,
  Spread,
} from 'lexical';


interface SerializedBannerProperties {}
export type SerializedBannerNode = Spread<SerializedBannerProperties, SerializedElementNode>;

export class BannerNode extends ElementNode {
  public static importJSON(_: SerializedBannerNode): BannerNode {
    return new BannerNode();
  }

  public static clone(node: BannerNode): BannerNode {
    return new BannerNode(node.__key);
  }

  public static getType(): string {
    return '--banner--';
  }

  public createDOM(_config: EditorConfig, _editor: LexicalEditor): HTMLElement {
    const div = document.createElement('div');
    div.className = _config.theme.banner;
    return div;
  }

  public updateDOM(
    _prevNode: unknown, _dom: HTMLElement, _config: EditorConfig,
  ): boolean {
    /**
     * Returning false tells Lexical that this node does not need its
     * DOM element replacing with a new copy from createDOM.
     */
    return false;
  }

  public insertNewAfter(_selection: RangeSelection, restoreSelection?: boolean): null | LexicalNode {
    const paragraph = $createParagraphNode();
    const direction = this.getDirection();
    paragraph.setDirection(direction);
    this.insertAfter(paragraph, restoreSelection);

    return paragraph;
  }

  public collapseAtStart(_selection: RangeSelection): boolean {
    const paragraph = $createParagraphNode();
    const children = this.getChildren();
    children.forEach((child) => paragraph.append(child));
    this.replace(paragraph);

    return true;
  }

  public exportJSON(): SerializedBannerNode {
    return {
      type: 'banner',
      version: 1,
      children: [],
      format: '',
      indent: 1,
      direction: null,
    };
  }
}

export const $createBannerNode = (): BannerNode => new BannerNode();
export const $isBannerNode = (node: LexicalNode | null | undefined): node is BannerNode => node instanceof BannerNode;
