/* eslint-disable @typescript-eslint/no-explicit-any */
import { AssistantResponseBuilder, NonFunctionProperties, default_thread_id } from './assistant-response-builder';


type ListNoThreadSingleMessagesMockProps = NonFunctionProperties<ListNoThreadSingleMessageMock>;


export class ListNoThreadSingleMessageMock extends AssistantResponseBuilder {
  public threadId?: string = default_thread_id;

  public constructor(props: ListNoThreadSingleMessagesMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(url: string, config: RequestInit): boolean {
    const doesMatch = config.method?.toLowerCase() === 'get' && url.endsWith(`/v1/threads/${this.threadId}/messages`);
    return doesMatch;
  }

  public getResponseJson(_requestBody: any): object {
    return {
      'object': 'list',
      'data': [],
      'first_id': undefined,
      'last_id': undefined,
      'has_more': false
    };

  }
}
