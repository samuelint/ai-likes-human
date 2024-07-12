import { useToast } from '@/components/ui/use-toast';
import { useCallback, useEffect } from 'react';


export type ErrorNotificationFn = (error?: Error | unknown | null) => void;
export function useErrorNotificationFn(): ErrorNotificationFn {
  const { toast } = useToast();


  return useCallback<ErrorNotificationFn>((error?: Error | unknown | null) => {
    if (error instanceof Error) {
      toast({
        title: 'Uh oh! Something went wrong (from hook value).',
        description: error.message,
        variant: 'destructive',
      });
    }
  }, [toast]);
}

export function useErrorNotification(error?: Error | unknown | null) {
  const fn = useErrorNotificationFn();

  useEffect(() => {
    fn(error);
  }, [fn, error]);
}

