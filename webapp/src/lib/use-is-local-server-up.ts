import { useIsInDesktopAppFn } from './is-in-desktop-app';
import { isInferenceServerRunning } from './tauri-command';
import { useCallback, useEffect, useState } from 'react';


interface Props {
  refreshInterval?: number;
}

export function useIsLocalServerUp ({ refreshInterval = 10000 }: Props = {}) {
  const isDesktopAppFn = useIsInDesktopAppFn();
  const [isUp, setIsUp] = useState(false);
  const [hasAlreadyBeenUp, setHasAlreadyBeenUp] = useState(false);

  const update = useCallback(async () => {
    const newIsUp = await isInferenceServerRunning();
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