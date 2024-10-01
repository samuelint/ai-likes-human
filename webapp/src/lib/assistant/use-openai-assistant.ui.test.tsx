/**
 * @jest-environment jsdom
 */
import 'whatwg-fetch';
import '@testing-library/jest-dom';
import { when } from 'jest-when';
import { cleanup, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import OpenAI from 'openai';
import { useOpenAiAssistant } from './use-openai-assistant';
import { buildOpenAiApiFetchMock, CreateMessageMock, CreateRunMock, CreateThreadMock, ErrorMock } from '@/lib/assistant/openai-fetch-mock';
import { useOpenaiClient } from './openai-client';
import { useState } from 'react';
import { ListThreadSingleMessageMock } from './openai-fetch-mock/list-thread-message-mock';


vi.mock('./openai-client');
describe('new-conversation', () => {
  const fetch = vi.fn();
  const TestComponent = () => {
    const threadId = 'thread_abc123';
    when(useOpenaiClient).mockReturnValue(new OpenAI({
      apiKey: 'abc',
      fetch,
      dangerouslyAllowBrowser: true,
    }));

    const { status, messages, error, append, abort } = useOpenAiAssistant({ threadId });
    const [appendError, setAppendError] = useState<Error>();

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
          onClick={async () => {
            try {
              await append({ role: 'user', content: 'Hello AI' });
            } catch (e) {
              setAppendError(e as Error);
            }
          }}
        />
        <button
          data-testid="abort"
          onClick={() => abort()}
        />

        { appendError && <div data-testid="append-error">{appendError.toString()}</div>}
      </div>
    );
  };

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  describe('show messages', () => {
    beforeEach(() => {
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

      render(<TestComponent />);
    });

    it('should show appended message role', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('message-0');
      expect(screen.getByTestId('message-0')).toHaveTextContent('Role: user');
    });

    it('should show appended message content', async () => {
      await userEvent.click(screen.getByTestId('do-append'));

      await screen.findByTestId('message-0');

      await waitFor(async () => {
        expect(screen.getByTestId('message-0')).toHaveTextContent('Hello AI');
      });
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

    it('should not submit when in progress', async () => {
      await userEvent.click(screen.getByTestId('do-append'));
      await waitFor(async () => {
        expect(screen.getByTestId('status')).toHaveTextContent('in_progress');
      });

      await userEvent.click(screen.getByTestId('do-append'));


      await waitFor(async () => {
        expect(screen.getByTestId('message-0')).toHaveTextContent('Hello AI');
        expect(screen.getByTestId('message-1')).toHaveTextContent('Hello human');
        expect(screen.queryByTestId('message-2')).not.toBeInTheDocument();
      });

      await waitFor(async () => {
        expect(screen.queryByTestId('append-error')).toBeInTheDocument();
      });
    });
  });

  describe('error', () => {
    beforeEach(() => {
      render(<TestComponent />);
    });

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
        new CreateThreadMock(),
        new CreateRunMock({
          finishGenerationPromise: new Promise(resolve => {
            finishGeneration = resolve;
          })
        }),

      ]));

      render(<TestComponent />);
    });

    it('should show loading state', async () => {
      await userEvent.click(screen.getByTestId('do-append'));
      await waitFor(async () => {
        expect(screen.getByTestId('status')).toHaveTextContent('in_progress');
      });

      finishGeneration?.();

      await waitFor(async () => {
        expect(screen.getByTestId('status')).toHaveTextContent('awaiting_message');
      });
    });
  });

  describe('abort', () => {
    beforeEach(() => {
      fetch.mockImplementation(buildOpenAiApiFetchMock([
        new CreateMessageMock(),
        new CreateThreadMock(),
        new CreateRunMock({
          finishGenerationPromise: new Promise(() => {
            // Never resolve, will be resolved by abort controller
          })
        }),
      ]));

      render(<TestComponent />);
    });

    it('should stop generation', async () => {
      await userEvent.click(screen.getByTestId('do-append'));
      await waitFor(async () => expect(screen.getByTestId('status')).toHaveTextContent('in_progress'));

      await userEvent.click(screen.getByTestId('abort'));

      await waitFor(async () => expect(screen.getByTestId('status')).toHaveTextContent('awaiting_message'), { timeout: 1000 });
    });
  });
});

describe('existing-thread', () => {
  const threadId = 'thread_abc123';
  const fetch = vi.fn();
  const TestComponent = () => {
    when(useOpenaiClient).mockReturnValue(new OpenAI({
      apiKey: 'abc',
      fetch,
      dangerouslyAllowBrowser: true,
    }));

    const { status, messages, error, append } = useOpenAiAssistant({ threadId });

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

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  describe('with last message from user', () => {
    beforeEach(() => {

      fetch.mockImplementation(buildOpenAiApiFetchMock([
        new ListThreadSingleMessageMock({
          messageId: 'msg_abc123',
          threadId,
          role: 'user',
          text_content: 'How does AI work? Explain it in simple terms.',
        }),
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
                'status': 'completed',
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
      ]));

      render(<TestComponent />);
    });

    it('should show existing message content', async () => {
      await waitFor(async () => {
        await screen.findByTestId('message-0');

        expect(screen.getByTestId('message-0')).toHaveTextContent('How does AI work? Explain it in simple terms.');
      });
    });

    it('should start run automatically', async () => {
      await waitFor(async () => {
        await screen.findByTestId('message-1');
        expect(screen.getByTestId('message-1')).toHaveTextContent('Hello human');
      });
    });
  });

  describe('with last message from user and error on run', () => {
    let runCallMock: ErrorMock;
    beforeEach(() => {
      runCallMock = new ErrorMock({ status: 400, errorMessage: 'Not found', url_match: '/runs' });

      fetch.mockImplementation(buildOpenAiApiFetchMock([
        new ListThreadSingleMessageMock({
          messageId: 'msg_abc123',
          threadId,
          role: 'user',
          text_content: 'How does AI work? Explain it in simple terms.',
        }),
        runCallMock,
      ]));

      render(<TestComponent />);
    });

    it('should have an error', async () => {
      await screen.findByTestId('error');
      expect(screen.getByTestId('error')).toHaveTextContent(
        'Error: 400 Not found',
      );
    });

    it('should automatically start a new run only once', async () => {
      await waitFor(async () => {
        expect(runCallMock.callCount).toBe(1);
      });

      // There's no way to test that the value is never greater than 1 in this context.
      // The callCount is 1 at some point, then after some times it becomes 2, and 3, and so on..
      // When doing:
      //
      // await waitFor(async () => {
      //   expect(runCallMock.callCount).toBe(1);
      // });
      // The test pass since the counter is set to one. But that does not test that it does not increment further.
      // This is why there's a wait here.
      await new Promise((resolve) => setTimeout(resolve, 1000));
      expect(runCallMock.callCount).toBe(1);
    });
  });
});
