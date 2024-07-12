import { BaseSelection, LexicalNode } from 'lexical';

/* eslint-disable @typescript-eslint/no-explicit-any */
type Newable<T> = { new (args: any): T };
/* eslint-enable @typescript-eslint/no-explicit-any */
export function $findSelectionNodeUpToRoot<TNode extends LexicalNode>(selection: BaseSelection | null, clazz: Newable<TNode>): TNode | undefined {
  if (!selection) return undefined;
  const nodes = selection.getNodes();
  const nodesToRoot = nodes.concat(nodes.flatMap((node) => node.getParents()));

  return nodesToRoot.find((node) => node instanceof clazz) as TNode | undefined;
}

export function $findNodeUpToRoot<TNode extends LexicalNode>(node: LexicalNode, clazz: Newable<TNode>): TNode | null {
  if (node instanceof clazz) return node;

  const parent = node.getParent();
  if (!parent) return null;

  return $findNodeUpToRoot(parent, clazz);
}
