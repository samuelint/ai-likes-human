import appConfig from '@/app.config';


interface CreateApiJsonFetcherArgs {
  queryParams?: Record<string, string>
}

export class FetchError extends Error {
  public constructor(message: string, public readonly status: number) {
    super(message);
  }
}

export async function fetchApi(path: string, init?: RequestInit) {
  const result = await fetch(`${appConfig.api_url}${path}`, init);
  if (!result.ok) {
    const msg = await result.text();
    const error = new FetchError(msg, result.status);

    throw error;
  }

  return result;
}

export async function fetchApiJson<T = object>(path: string, init?: RequestInit): Promise<T> {
  const result = await fetchApi(path, { ...init, headers: { ...init?.headers, 'Content-Type': 'application/json' } });

  return await result.json();
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function createApiJsonFetcher(args: CreateApiJsonFetcherArgs = {}): (path: string) => Promise<any> {
  if (args?.queryParams) {
    const searchParams = new URLSearchParams(args.queryParams);
    return (path) => fetchApiJson(`${path}?${searchParams.toString()}`);
  } else {
    return (path) => fetchApiJson(path);
  }
}
