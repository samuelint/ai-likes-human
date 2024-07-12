import { useEffect, useRef } from 'react';
import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';


export function useLocalServerLogForward() {
  const unlistednRef = useRef<UnlistenFn | null>(null);

  useEffect(() => {
    appWindow.listen('local-server-stderr', (event) => {
      console.debug(event.payload);
    }).then((unlisten) => {
      unlistednRef.current = unlisten;
    });

    return () => {
      if (unlistednRef.current) {
        unlistednRef.current();
        unlistednRef.current = null;
      }
    };
  }, []);
}
