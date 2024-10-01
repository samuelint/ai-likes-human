import { isAtLeastOneLlmApiKeySet } from '@/lib/api/llm';
import { useAsyncRetry, useInterval } from 'react-use';

interface Props {
  refreshInterval?: number;
}

export function useMustOnboard({ refreshInterval = 1000 }: Props = {}) {

  const state = useAsyncRetry(async () => {
    const response = await isAtLeastOneLlmApiKeySet();
    return !response;
  }, []);

  useInterval(
    () => {
      state.retry();
    },
    state.value === true ? refreshInterval : null
  );

  return state;
}