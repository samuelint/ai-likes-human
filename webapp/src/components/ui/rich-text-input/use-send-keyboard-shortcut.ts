import { OS, useCurrentOS } from '@/lib/utils/use-current-os';
import { useMemo } from 'react';

export function useSendKeyboardShortcut(): string[] {
  const os = useCurrentOS();

  return useMemo(() => {
    switch (os) {
    case OS.Mac:
      return ['⌘', '↵'];
    default:
      return ['Ctrl', '↵'];
    }

  }, [os]);
}