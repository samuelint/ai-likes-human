/* eslint-disable @typescript-eslint/no-explicit-any */
import { AssistantResponseBuilder } from './assistant-response-builder';


if (!global.fetch) global.fetch = vi.fn();


export function buildOpenAiApiFetchMock(builders: AssistantResponseBuilder[]) {
  return async (url: string, config: RequestInit) => {
    const requestBody = JSON.parse(config.body as string);
    for (const builder of builders) {
      if (builder.doesMatch(url, config)) {
        return builder.getResponse(requestBody);
      }
    }
  };
}
