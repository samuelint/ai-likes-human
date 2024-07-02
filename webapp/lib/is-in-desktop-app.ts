'use client';
import { useCallback } from 'react';


export function useIsInDesktopApp() {
  return useCallback(() => {
    return global?.window?.__TAURI_IPC__ != null;
  }, []);

}