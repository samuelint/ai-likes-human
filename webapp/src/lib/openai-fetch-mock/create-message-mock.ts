/* eslint-disable @typescript-eslint/no-explicit-any */
import { AssistantResponseBuilder, NonFunctionProperties, default_message_id, default_thread_id } from './assistant-response-builder';


type CreateMessageMockProps = NonFunctionProperties<CreateMessageMock>;


export class CreateMessageMock extends AssistantResponseBuilder {
  public threadId?: string = default_thread_id;
  public role?: string;
  public text_content?: string;
  public created_at?: number = 1713226573;

  public messageId?: string = (Math.random() + 1).toString(36).substring(7);

  public constructor(props: CreateMessageMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(url: string, config: RequestInit): boolean {
    return config.method?.toLowerCase() === 'post' && url.endsWith(`/v1/threads/${this.threadId}/messages`);
  }

  public getResponseJson(requestBody: any): object {
    console.log({ this: this, requestBody });
    return {
      'id': this.messageId,
      'object': 'thread.message',
      'created_at': this.created_at,
      'thread_id': this.threadId,
      'role': this.role ? this.role : requestBody.role,
      'content': [{
        'type': 'text',
        'text': {
          'value': this.text_content ? this.text_content : requestBody.content,
          'annotations': []
        }
      }],
      'attachments': [],
      'metadata': {}
    };
  }
}
