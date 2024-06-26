import { createPortal } from 'react-dom';
import { CodeActions, CodeActionsChildren } from './code-actions.component';


interface Props {
  anchorElem?: HTMLElement;
  children?: CodeActionsChildren;
}

export default function CodeActionMenuPlugin({
  anchorElem = document.body,
  children,
}: Props): React.ReactPortal | null {
  return createPortal(
    // eslint-disable-next-line react/no-children-prop
    <CodeActions anchorElem={anchorElem} children={children} />,
    anchorElem,
  );
}