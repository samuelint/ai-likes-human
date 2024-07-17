import { isLocalServerRunning } from './tauri-interrupt/server-status';
import { useCallback, useEffect, useState } from 'react';


interface Props {
  refreshInterval?: number;
}

export function useIsLocalServerUp ({ refreshInterval = 2000 }: Props = {}) {
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
    update();
  // Update at mount
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const id = setInterval(async () => {
      update();
    }, refreshInterval);
    return () => clearInterval(id);
  }, [update, refreshInterval]);

  return {
    isUp,
    hasAlreadyBeenUp
  };
}