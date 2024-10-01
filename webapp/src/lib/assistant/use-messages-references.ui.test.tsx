/**
 * @jest-environment jsdom
 */
import OpenAI from 'openai';
import '@testing-library/jest-dom';

import { cleanup, render, screen } from '@testing-library/react';
import { useMessagesReferences } from './use-messages-references';



describe('', () => {
  const TestComponent = ({ messages }: { messages: Pick<OpenAI.Beta.Threads.Messages.Message, 'role' | 'content'>[] }) => {
    const references = useMessagesReferences({ messages });

    return (
      <div>
        {references.map((reference, idx) => (
          <div data-testid={`reference-${idx}`} key={idx}>
            <span>Title: {reference.title}</span>
            <span>Link: {reference.link}</span>
          </div>
        ))}
      </div>
    );
  };

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  it('list every links from content of assistant messages', async () => {
    render(<TestComponent messages={[{
      role: 'assistant',
      content: [{
        text: {
          annotations: [],
          value: `
# Title
Hello, this is an answer.

### Some ref
- **spacy**: [Spacy Documentation](https://spacy.io/usage)

#### Some other refs
- **transformers**: [Transformers Documentation](https://huggingface.co/transformers/)
          `,
        },
        type: 'text',
      }],
    }]} />);

    expect(screen.getByTestId('reference-0')).toHaveTextContent('Title: Spacy Documentation');
    expect(screen.getByTestId('reference-0')).toHaveTextContent('Link: https://spacy.io/usage');

    expect(screen.getByTestId('reference-1')).toHaveTextContent('Title: Transformers Documentation');
    expect(screen.getByTestId('reference-1')).toHaveTextContent('Link: https://huggingface.co/transformers/');
  });

  it('list ONLY links from assistant messages', async () => {
    render(<TestComponent messages={[
      {
        role: 'user',
        content: [{
          text: {
            annotations: [],
            value: `
Hi, from this ref:
- **spacy**: [Spacy Documentation](https://spacy.io/usage)

give an alternative`,
          },
          type: 'text',
        }],
      },
      {
        role: 'assistant',
        content: [{
          text: {
            annotations: [],
            value: `
# Title
Ok heres another ref

#### Some other refs
- **transformers**: [Transformers Documentation](https://huggingface.co/transformers/)
          `,
          },
          type: 'text',
        }],
      },
    ]} />);

    expect(screen.getByTestId('reference-0')).toHaveTextContent('Title: Transformers Documentation');
    expect(screen.getByTestId('reference-0')).toHaveTextContent('Link: https://huggingface.co/transformers/');
  });
});
