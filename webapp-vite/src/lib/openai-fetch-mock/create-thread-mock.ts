
import { AssistantResponseBuilder, NonFunctionProperties, default_thread_id } from './assistant-response-builder';


type CreateThreadMockProps = NonFunctionProperties<CreateThreadMock>;


export class CreateThreadMock extends AssistantResponseBuilder {
  public threadId?: string = default_thread_id;

  public constructor(props: CreateThreadMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(url: string, config: RequestInit): boolean {
    return config.method?.toLowerCase() === 'post' && url.endsWith('/v1/threads');
  }

  public getResponseJson(): object {
    return {
      'id': this.threadId,
      'object': 'thread',
      'created_at': 1699012949,
      'metadata': {},
      'tool_resources': {}
    };
  }
}
