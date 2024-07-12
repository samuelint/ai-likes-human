import { ToolbarAction } from '../../toolbar';
import { FORMAT_HEADING_COMMAND } from './heading.command';


export function ParagraphAction() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_HEADING_COMMAND,
        payload: 'p'
      }}
    >
      Normal
    </ToolbarAction>
  );
}

export function H1Action() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_HEADING_COMMAND,
        payload: 'h1'
      }}
    >
      H1
    </ToolbarAction>
  );
}

export function H2Action() {
  return (
    <ToolbarAction
      action={{
        command: FORMAT_HEADING_COMMAND,
        payload: 'h2'
      }}
    >
      H2
    </ToolbarAction>
  );
}

export function HeadingToolbarActions() {
  return (
    <>
      <ParagraphAction />
      <H1Action />
      <H2Action />
    </>
  );
}
