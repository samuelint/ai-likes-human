
import { AssistantResponseBuilder } from './assistant-response-builder';


if (!global.fetch) global.fetch = vi.fn();


export function buildOpenAiApiFetchMock(builders: AssistantResponseBuilder[], delay: number = 100) {
  return async (url: string, config: RequestInit) => {
    const requestBody = JSON.parse(config.body as string);
    for (const builder of builders) {
      if (builder.doesMatch(url, config)) {
        await new Promise(resolve => setTimeout(resolve, delay));
        return builder.getResponse(requestBody);
      }
    }
  };
}
