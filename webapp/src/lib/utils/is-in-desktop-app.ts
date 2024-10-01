
import { useCallback, useEffect, useState } from 'react';


export function useIsInDesktopAppFn() {
  return useCallback(() => {
    return window?.__TAURI_IPC__ != null;
  }, []);
}

export function useIsInDesktopApp() {
  const isInDesktopAppFn = useIsInDesktopAppFn();
  const [isInDesktopApp, setIsInDesktopApp] = useState(false);

  useEffect(() => {
    setIsInDesktopApp(isInDesktopAppFn());
  }, [isInDesktopAppFn]);

  return isInDesktopApp;
}