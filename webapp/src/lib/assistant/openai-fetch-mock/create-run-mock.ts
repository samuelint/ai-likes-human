
import { AssistantResponseBuilder, NonFunctionProperties, default_thread_id } from './assistant-response-builder';


type CreateRunMockProps = NonFunctionProperties<CreateRunMock>;

interface EventChunkMock {
  event: string
  data: object
}

// Reference: https://platform.openai.com/docs/api-reference/assistants-streaming/message-delta-object
const default_chunks = [
  { event: 'thread.run.created',
    data: {
      'id': 'run_abc123',
      'object': 'thread.run',
      'created_at': 1699063290,
      'assistant_id': 'asst_abc123',
      'thread_id': 'thread_abc123',
      'status': 'queued',
      'started_at': 1699063290,
      'completed_at': 1699063291,
      'model': 'gpt-4-turbo',
      'tools': [],
      'metadata': {},
      'temperature': 1.0,
      'top_p': 1.0,
      'max_prompt_tokens': 1000,
      'max_completion_tokens': 1000,
      'response_format': 'auto',
      'tool_choice': 'auto',
    }
  },
  {
    event: 'thread.message.created',
    data: {
      'id': 'msg_123',
      'object': 'thread.message',
      'created_at': 1713226573,
      'assistant_id': 'asst_abc123',
      'thread_id': 'thread_abc123',
      'run_id': 'run_abc123',
      'role': 'assistant',
      'content': [
      ],
      'attachments': [],
      'metadata': {}
    }
  },
  {
    event: 'thread.message.delta',
    data: {
      'id': 'msg_123',
      'object': 'thread.message.delta',
      'delta': {
        'content': [
          {
            'index': 0,
            'type': 'text',
            'text': { 'value': 'Hello human', 'annotations': [] }
          }
        ]
      }
    }
  },
  {
    event: 'thread.message.completed',
    data: {
      'id': 'msg_123',
      'object': 'thread.message',
      'created_at': 1698983503,
      'thread_id': 'thread_abc123',
      'role': 'assistant',
      'content': [
        {
          'index': 0,
          'type': 'text',
          'text': {
            'value': 'Hello human',
            'annotations': []
          }
        }
      ],
      'assistant_id': 'asst_abc123',
      'run_id': 'run_abc123',
      'attachments': [],
      'metadata': {}
    }
  },
  {
    event: 'thread.run.completed',
    data: {
      'id': 'run_abc123',
      'object': 'thread.run',
      'created_at': 1699063290,
      'assistant_id': 'asst_abc123',
      'thread_id': 'thread_abc123',
      'status': 'queued',
      'started_at': 1699063290,
      'completed_at': 1699063291,
      'model': 'gpt-4-turbo',
      'tools': [],
      'metadata': {},
      'usage': null,
      'temperature': 1.0,
      'top_p': 1.0,
      'max_prompt_tokens': 1000,
      'max_completion_tokens': 1000,

      'response_format': 'auto',
      'tool_choice': 'auto',
    }
  }];

export function encodeChunk(chunk: EventChunkMock): Uint8Array {
  return new TextEncoder().encode(`event: ${chunk.event}\ndata: ${JSON.stringify(chunk.data)}\n\n`);
}

export class CreateRunMock extends AssistantResponseBuilder {
  public threadId?: string = default_thread_id;
  public chunks?: EventChunkMock[] = default_chunks;
  public finishGenerationPromise?: Promise<void>;

  public constructor(props: CreateRunMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(url: string, config: RequestInit): boolean {
    return config.method?.toLowerCase() === 'post' && url.endsWith(`/v1/threads/${this.threadId}/runs`);
  }


  public getResponse(_requestBody: never, config: RequestInit): Response {
    const chunks = this.chunks ?? [];
    const finishGenerationPromise = this.finishGenerationPromise;
    async function* generateChunks() {
      for (const chunk of chunks) {
        yield encodeChunk(chunk);
      }

      if (finishGenerationPromise)
        await finishGenerationPromise;
    }

    const chunkGenerator = generateChunks();

    return {
      ok: true,
      status: 200,
      bodyUsed: false,
      headers: new Map([['content-type', 'text/event-stream']]),
      body: new ReadableStream({
        async start(controller) {
          if (config && config.signal) {
            if (config.signal.aborted) {
              controller.error(new DOMException('Aborted', 'AbortError'));
              return;
            }
            config.signal.addEventListener('abort', () => {
              controller.error(new DOMException('Aborted', 'AbortError'));
            });
          }

          try {
            for await (const chunk of chunkGenerator) {
              controller.enqueue(chunk);
            }
            controller.close();
          } catch (error) {
            controller.error(error);
          }
        },
      }),
    } as unknown as Response;
  }
}