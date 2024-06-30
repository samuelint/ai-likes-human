import appConfig from '@/app.config';


export function apiJsonFetch(url: string, init?: RequestInit): Promise<Response> {
  return fetch(`${appConfig.api_url}${url}`, { ...init, headers: { ...init?.headers, 'Content-Type': 'application/json' } });
}

export const apiJsonFetcher = (url: string) => apiJsonFetch(url).then(res => res.json());
