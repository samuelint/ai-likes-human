import { AddImageAttachments } from '@/lib/assistant/image-attachment.type';
import { AddScreenshotToInput } from './add-screenshot-to-input';
import { useIsInDesktopApp } from '@/lib/utils';


interface Props {
  addImageAttachments: AddImageAttachments
}

export function Tools({ addImageAttachments }: Props) {
  const isInDesktopApp = useIsInDesktopApp();
  return (
    <>
      { isInDesktopApp && <AddScreenshotToInput addImageAttachments={addImageAttachments} />}
    </>
  );
}