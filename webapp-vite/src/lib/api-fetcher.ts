import appConfig from '@/app.config';


interface CreateApiJsonFetcherArgs {
  queryParams?: Record<string, string>
}

export function fetchApi(path: string, init?: RequestInit) {
  return fetch(`${appConfig.api_url}${path}`, init);
}

export async function fetchApiJson(path: string, init?: RequestInit) {
  const result = await fetchApi(path, { ...init, headers: { ...init?.headers, 'Content-Type': 'application/json' } });
  if (!result.ok) {
    throw new Error(await result.text());
  }

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
