// export const apiFetcher = async (input: string | URL | globalThis.Request, init?: RequestInit) => {
//   // @ts-expect-error - ... args is accepted
//   const response = await fetch(...args);
//   if (!response.ok) {
//     throw new Error('Network response was not ok');
//   }
//   return response.json();
// };


const BASE_URL = 'http://localhost:8000';

// @ts-expect-error - Ok for now
export const apiFetcher = (url) => fetch(`${BASE_URL}${url}`).then(res => res.json());
