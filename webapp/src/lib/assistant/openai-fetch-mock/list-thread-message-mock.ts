/* eslint-disable @typescript-eslint/no-explicit-any */
import { AssistantResponseBuilder, NonFunctionProperties, default_message_id, default_thread_id } from './assistant-response-builder';


type ListThreadSingleMessagesMockProps = NonFunctionProperties<ListThreadSingleMessageMock>;


export class ListThreadSingleMessageMock extends AssistantResponseBuilder {
  public threadId?: string = default_thread_id;
  public role: string = 'user';
  public text_content?: string;
  public created_at?: number = 1713226573;

  public messageId?: string = default_message_id;

  public constructor(props: ListThreadSingleMessagesMockProps = {
    role: 'user',
  }) {
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
      'data': [{
        'id': this.messageId,
        'object': 'thread.message',
        'created_at': this.created_at,
        'thread_id': this.threadId,
        'role': this.role,
        'content': [{
          'type': 'text',
          'text': {
            'value': this.text_content,
            'annotations': []
          }
        }],
        'attachments': [],
        'metadata': {}
      }],
      'first_id': this.messageId,
      'last_id': this.messageId,
      'has_more': false
    };

  }
}
