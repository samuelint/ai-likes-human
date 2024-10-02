import { fileToBase64 } from '../utils/file-to-base64';
import { useClipboardPasteEvent } from '../utils/use-clipboard-paste-event';
import { ImageAttachment } from './image-attachment.type';

type OnImagePasteEvent = (image: ImageAttachment) => void;

export function useClipboardPasteImage(onEvent: OnImagePasteEvent) {
  useClipboardPasteEvent(async (event) => {
    if (event.type === 'image') {
      const base64 = await fileToBase64(event.image);
      onEvent({
        title: event.image.name,
        base64,
      });
    }
  });
}