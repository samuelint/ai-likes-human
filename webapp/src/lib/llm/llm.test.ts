import { when } from 'jest-when';
import { isAtLeastOneLlmApiKeySet } from './llm';
import { findConfiguration } from '@/lib/core-api/tauri';
import { LLM_API_KEYS_KEYS } from '@/app.config';

vi.mock('@/lib/core-api/tauri');
describe('isAtLeastOneLlmApiKeySet', () => {

  it('should return false when each api key value is empty', async () => {
    LLM_API_KEYS_KEYS.forEach((key) => {
      when(findConfiguration).mockResolvedValue({
        key: key,
        value: '',
      });
    });

    const result = await isAtLeastOneLlmApiKeySet();

    expect(result).toBe(false);
  });

  it('should return true when one of api key value is not empty', async () => {
    when(findConfiguration).mockResolvedValue({
      key: '',
      value: '',
    });

    when(findConfiguration).calledWith(LLM_API_KEYS_KEYS[0]).mockResolvedValue({
      key: '',
      value: 'some',
    });

    const result = await isAtLeastOneLlmApiKeySet();

    expect(result).toBe(true);
  });
});