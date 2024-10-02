import { cleanup, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useConfigurationKV } from './use-configuration-kv';
import { findConfiguration, upsertConfiguration } from '@/lib/core-api/tauri';
import { when } from 'jest-when';

vi.mock('@/lib/core-api/tauri');
describe('stream data stream', () => {
  const TestComponent = () => {
    const { data, error, mutate } = useConfigurationKV('some-key');

    return (
      <div>
        <div data-testid="data">{data?.value}</div>
        <div data-testid="error">{JSON.stringify(error)}</div>

        <button
          data-testid="mutate"
          onClick={() => {
            mutate('mutated-value');
          }}
        />
      </div>
    );
  };

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  it('should have value for key', async () => {
    when(findConfiguration).calledWith('some-key').mockResolvedValue({
      key: 'some-key',
      value: 'some-value',
    });

    render(<TestComponent />);

    await screen.findByTestId('data');

    expect(screen.getByTestId('data')).toHaveTextContent('some-value');
  });

  it('should display mutated value', async () => {
    when(findConfiguration).calledWith('some-key').mockResolvedValue({
      key: 'some-key',
      value: 'some-value',
    });
    when(upsertConfiguration).calledWith({ key: 'some-key', value: 'mutated-value' }).mockResolvedValue({
      key: 'some-key',
      value: 'mutated-value',
    });

    render(<TestComponent />);

    await userEvent.click(screen.getByTestId('mutate'));

    await screen.findByTestId('data');
    expect(screen.getByTestId('data')).toHaveTextContent('mutated-value');
  });
});
