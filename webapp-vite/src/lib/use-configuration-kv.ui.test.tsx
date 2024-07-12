import { cleanup, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useConfigurationKV } from './use-configuration-kv';


describe('stream data stream', () => {
  const fetch = vi.fn();
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

  beforeEach(() => {
    global.fetch = fetch;
    fetch.mockImplementation(async (url) => {
      if (url.endsWith('/configuration/kv/some-key')) {
        return {
          ok: true,
          status: 200,
          json: async () => ({
            key: 'some-key',
            value: 'some-value',
          })
        } as unknown as Response;
      }
    });

    render(<TestComponent />);
  });

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  it('should have value', async () => {
    await screen.findByTestId('data');
    expect(screen.getByTestId('data')).toHaveTextContent('some-value');
  });

  it('should display mutated value', async () => {
    await userEvent.click(screen.getByTestId('mutate'));

    await screen.findByTestId('data');
    expect(screen.getByTestId('data')).toHaveTextContent('mutated-value');
  });

  it('should mutate value', async () => {
    await userEvent.click(screen.getByTestId('mutate'));

    expect(fetch).toHaveBeenCalledWith(
      expect.stringContaining('/configuration/kv/some-key'),
      expect.objectContaining({
        method: 'PUT',
        body: JSON.stringify({
          key: 'some-key',
          value: 'mutated-value',
        }),
      })
    );
  });
});
