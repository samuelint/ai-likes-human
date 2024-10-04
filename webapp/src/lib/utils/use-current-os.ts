import { useMemo } from 'react';

export enum OS {
  Unknown,
  Windows,
  Mac,
  Linux,
}

export function useCurrentOS(): OS {
  return useMemo(() => {
    const userAgent = navigator.userAgent.toLowerCase();

    if (userAgent.includes('win')) {
      return OS.Windows;
    }

    if (userAgent.includes('mac')) {
      return OS.Mac;
    }

    if (userAgent.includes('linux')) {
      return OS.Linux;
    }

    return OS.Unknown;

  }, []);
}