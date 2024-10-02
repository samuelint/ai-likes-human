import { useEffect } from 'react';



interface BasePasteEvent {
  type: string;
}

interface PasteTextEvent extends BasePasteEvent {
  type: 'text';
  text: string;
}

interface PasteImageEvent extends BasePasteEvent {
  type: 'image';
  image: File;
}

export type PasteEvent = PasteTextEvent | PasteImageEvent;

type OnPasteEvent = (event: PasteEvent) => void;

export const useClipboardPasteEvent = (onPaste: OnPasteEvent): void => {
  useEffect(() => {
    const handlePaste = (event: ClipboardEvent) => {
      if (typeof onPaste === 'function') {

        const clipboardItems = event.clipboardData?.items ?? [];

        for (const item of clipboardItems) {
          if (item.type === 'text/plain') {
            item.getAsString((data) => {
              onPaste({
                type: 'text',
                text: data,
              });
            });

          }
          else if (item.type.startsWith('image/')) {
            const blob = item.getAsFile();
            if (blob) {
              onPaste({
                type: 'image',
                image: blob,
              });
            }
          }
        }
      }
    };

    document.addEventListener('paste', handlePaste);

    return () => {
      document.removeEventListener('paste', handlePaste);
    };
  }, [onPaste]);
};
