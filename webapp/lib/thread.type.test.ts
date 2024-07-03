import { Thread } from 'openai/resources/beta/threads/threads.mjs';
import { toThreadPreview } from './thread.type';


describe('toThreadPreview', () => {
  it('should convert a thread to a thread preview', () => {
    const thread = {
      id: '123',
      created_at: 123456789,
      metadata: {
        'title': 'Thread 1'
      }
    } as Thread;

    const result = toThreadPreview(thread);

    expect(result).toEqual({
      id: '123',
      created_at: new Date(123456789 * 1000),
      title: 'Thread 1'
    });
  });

  it('no metadata title should be Undefined', () => {
    const thread = {
      id: '123',
      created_at: 123456789,
    } as Thread;

    const result = toThreadPreview(thread);

    expect(result).toEqual({
      id: '123',
      created_at: new Date(123456789 * 1000),
      title: 'Undefined'
    });
  });
});
