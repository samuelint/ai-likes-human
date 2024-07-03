import '@testing-library/jest-dom';
import 'openai/shims/node';
import OpenAI from 'openai';
import { when } from 'jest-when';
import { cleanup, findByText, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useOpenAiAssistant } from './use-openai-assistant';
import { buildOpenAiApiFetchMock, CreateMessageMock, CreateRunMock, CreateThreadMock, ErrorMock } from '@/lib/openai-fetch-mock';
import { useOpenaiClient } from './use-openai-client';


jest.mock('./use-openai-client');
describe('stream data stream', () => {
  const fetch = jest.fn();
  const TestComponent = () => {
    when(useOpenaiClient).mockReturnValue(new OpenAI({
      apiKey: 'abc',
      fetch,
      dangerouslyAllowBrowser: true,
    }));
    const { status, messages, error, append } = useOpenAiAssistant();

    return (
      <div>
        <div data-testid="status">{status}</div>
        {error && <div data-testid="error">{error.toString()}</div>}
        {messages.map((m, idx) => (
          <div data-testid={`message-${idx}`} key={idx}>
            <span>Role: {m.role}</span>
            <span>{JSON.stringify(m.content)}</span>
          </div>
        ))}

        <button
          data-testid="do-append"
          onClick={() => {
            append({ role: 'user', content: 'Hello AI' });
          }}
        />
      </div>
    );
  };

  beforeEach(() => {
    render(<TestComponent />);
    fetch.mockImplementation(buildOpenAiApiFetchMock([
      new CreateMessageMock(),
      // Reference: https://platform.openai.com/docs/api-reference/assistants-streaming/message-delta-object
      new CreateRunMock({
        chunks: [
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
          }]
      }),
      new CreateThreadMock(),
    ]));
  });

  afterEach(() => {
    jest.restoreAllMocks();
    cleanup();
  });

  describe('show messages', () => {
    it('should show appended message role', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('message-0');
      expect(screen.getByTestId('message-0')).toHaveTextContent('Role: user');
    });

    it('should show appended message content', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('message-0');
      expect(screen.getByTestId('message-0')).toHaveTextContent('Hello AI');
    });

    it('should show streamed response role assistant', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await waitFor(async () => {
        await screen.findByTestId('message-1');
        expect(screen.getByTestId('message-1')).toHaveTextContent('Role: assistant');
      });
    });

    it('should show streamed response content', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await waitFor(async () => {
        await screen.findByTestId('message-1');
        expect(screen.getByTestId('message-1')).toHaveTextContent('Hello human');
      });
    });
  });

  describe('error', () => {
    it('should show error response', async () => {
      fetch.mockImplementation(buildOpenAiApiFetchMock([
        new ErrorMock({ status: 400, errorMessage: 'Not found' }),
      ]));

      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('error');
      expect(screen.getByTestId('error')).toHaveTextContent(
        'Error: 400 Not found',
      );
    });
  });

  describe('loading state', () => {
    let finishGeneration: (() => void) | undefined;

    beforeEach(() => {
      fetch.mockImplementation(buildOpenAiApiFetchMock([
        new CreateMessageMock(),
        new CreateRunMock({
          finishGenerationPromise: new Promise(resolve => {
            finishGeneration = resolve;
          })
        }),
        new CreateThreadMock(),
      ]));
    });
    it('should show loading state', async () => {

      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('status');
      expect(screen.getByTestId('status')).toHaveTextContent('in_progress');

      finishGeneration?.();

      await findByText(await screen.findByTestId('status'), 'awaiting_message');
      expect(screen.getByTestId('status')).toHaveTextContent(
        'awaiting_message',
      );
    });
  });
});
