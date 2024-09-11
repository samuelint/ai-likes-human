import { useIsInDesktopAppFn } from './is-in-desktop-app';
import { isLocalServerRunning } from './tauri-command/server-status';
import { useCallback, useEffect, useState } from 'react';


interface Props {
  refreshInterval?: number;
}

export function useIsLocalServerUp ({ refreshInterval = 2000 }: Props = {}) {
  const isDesktopAppFn = useIsInDesktopAppFn();
  const [isUp, setIsUp] = useState(false);
  const [hasAlreadyBeenUp, setHasAlreadyBeenUp] = useState(false);

  const update = useCallback(async () => {
    const newIsUp = await isLocalServerRunning();
    setIsUp(newIsUp);
    if (!hasAlreadyBeenUp && newIsUp) {
      setHasAlreadyBeenUp(true);
    }
  }, [hasAlreadyBeenUp]);

  useEffect(() => {
    if (!isDesktopAppFn()) return;
    update();
  // Update at mount
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!isDesktopAppFn()) return;
    const id = setInterval(async () => {
      update();
    }, refreshInterval);
    return () => clearInterval(id);
  }, [update, refreshInterval, isDesktopAppFn]);

  return {
    isUp,
    hasAlreadyBeenUp
  };
}