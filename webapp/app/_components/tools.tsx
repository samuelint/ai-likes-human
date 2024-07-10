'use client';
import { AddImageAttachments } from '@/lib/image-attachment.type';
import { AddScreenshotToInput } from './add-screenshot-to-input';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';


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