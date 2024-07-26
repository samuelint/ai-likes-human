/* eslint-disable @typescript-eslint/no-explicit-any */
export const default_message_id = 'msg_abc123';
export const default_thread_id = 'thread_abc123';


type NonFunctionPropertyNames<T> = {
  // eslint-disable-next-line @typescript-eslint/ban-types
  [K in keyof T]: T[K] extends Function ? never : K
}[keyof T];

export type NonFunctionProperties<T> = Pick<T, NonFunctionPropertyNames<T>>;



export abstract class AssistantResponseBuilder {
  public abstract doesMatch(url: string, config: RequestInit): boolean;
  public getResponseJson(_requestBody: any): object {
    return {};
  }

  public getResponse(requestBody: any, _config: RequestInit): Response {
    const jsonBody = this.getResponseJson(requestBody);
    return {
      ok: true,
      status: 200,
      headers: new Map([['content-type', 'application/json']]),
      json: async () => (jsonBody),
    } as unknown as Response;
  }
}