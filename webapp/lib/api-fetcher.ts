import appConfig from '@/app.config';


export function apiJsonFetch(url: string, init?: RequestInit): Promise<Response> {
  return fetch(`${appConfig.api_url}${url}`, { ...init, headers: { ...init?.headers, 'Content-Type': 'application/json' } });
}

export async function apiJsonFetcher(url: string) {
  const response = await apiJsonFetch(url);

  return await response.json();
}
