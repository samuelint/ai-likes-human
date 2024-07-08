'use client';
import { useCallback } from 'react';


export function useIsInDesktopAppFn() {
  return useCallback(() => {
    return global?.window?.__TAURI_IPC__ != null;
  }, []);
}

export function useIsInDesktopApp() {
  return useIsInDesktopAppFn()();
}