/* eslint-disable react-refresh/only-export-components */
import React, { createContext, useContext, ReactNode } from 'react';
import { useIsLocalServerUp } from './use-is-local-server-up';

interface ServerStatusContextType {
  isUp: boolean;
  hasAlreadyBeenUp: boolean;
}

const ServerStatusContext = createContext<ServerStatusContextType | undefined>(undefined);

interface ServerStatusProviderProps {
  children: ReactNode;
}

export const ServerStatusProvider: React.FC<ServerStatusProviderProps> = ({ children }) => {
  const { isUp, hasAlreadyBeenUp } = useIsLocalServerUp();

  return (
    <ServerStatusContext.Provider value={{ isUp, hasAlreadyBeenUp }}>
      {children}
    </ServerStatusContext.Provider>
  );
};

export function useServerStatus(): ServerStatusContextType {
  const context = useContext(ServerStatusContext);
  if (!context) {
    throw new Error('useServerStatus must be used within a SingletonProvider');
  }
  return context;
};
