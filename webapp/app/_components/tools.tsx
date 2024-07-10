import { AddScreenshotToInput } from './add-screenshot-to-input';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';


export function Tools() {
  const isInDesktopApp = useIsInDesktopApp();
  return (
    <>
      { isInDesktopApp && <AddScreenshotToInput/>}
    </>
  );
}