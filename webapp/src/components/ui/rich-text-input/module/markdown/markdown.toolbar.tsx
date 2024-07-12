import { ToolbarAction } from '../../toolbar';
import { APPLY_MARKDOWN_COMMAND } from './markdown.command';
import MarkdownIcon from './markdown.icon';


interface Props {
  className?: string
}

export function ToggleMarkdownAction({ className }: Props) {
  return (
    <ToolbarAction
      title='Markdown'
      action={{
        command: APPLY_MARKDOWN_COMMAND,
        payload: undefined
      }}
    >
      <MarkdownIcon className={className} />
    </ToolbarAction>
  );
}

export function MarkdownToolbarActions() {
  return (
    <>
      <ToggleMarkdownAction />
    </>
  );
}
