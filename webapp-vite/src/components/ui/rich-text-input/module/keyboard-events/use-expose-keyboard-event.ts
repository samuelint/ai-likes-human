import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { COMMAND_PRIORITY_CRITICAL, KEY_DOWN_COMMAND } from 'lexical';
import { useEffect } from 'react';



export type OnKeyboardEvent = (event: KeyboardEvent) => void;
export type OnKeyboardEventMapping = (event: KeyboardEvent) => boolean;
export interface UseExposeKeyboardEventProps {
  isKeyboardEvent: OnKeyboardEventMapping
  onSendEvent: OnKeyboardEvent
}

export function useExposeKeyboardEvent({ isKeyboardEvent, onSendEvent }: UseExposeKeyboardEventProps) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerCommand<KeyboardEvent | null>(
      KEY_DOWN_COMMAND,
      (event) => {
        if (!event || !onSendEvent) return false;

        const isMappedEvent = isKeyboardEvent(event);
        if (isMappedEvent) {
          onSendEvent(event);
        }
        return isMappedEvent;
      },
      COMMAND_PRIORITY_CRITICAL,
    );
  }, [editor, isKeyboardEvent, onSendEvent]);
}
