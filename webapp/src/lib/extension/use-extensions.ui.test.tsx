import { act, cleanup, render, screen, waitFor } from '@testing-library/react';
import { useExtensions } from './use-extensions';
import userEvent from '@testing-library/user-event';
import { SWRConfig } from 'swr';


describe('useExtensions', () => {
  const fetch = vi.fn();

  const mockNewExtension = new File(['file content'], 'new-extension.pex', {
    type: 'text/plain',
  });

  const TestComponent = () => {
    const { data, error, isLoading, upload, remove, load, unload } = useExtensions();

    return (
      <div>
        <ul>
          {data?.map((extension, index) => (
            <li data-testid={`ext-${index}`} key={extension.name}>
              {extension.name} - {extension.ipc_port}
            </li>
          ))}
        </ul>
        <div data-testid="loading">{isLoading ? 'true' : 'false'}</div>
        <div data-testid="error">{error?.message}</div>

        <button
          data-testid="upload"
          onClick={() => upload({ file: mockNewExtension })}
        />
        <button
          data-testid="removeExtensionZ"
          onClick={() => remove({ name: 'extensionZ' })}
        />
        <button
          data-testid="loadExtensionZ"
          onClick={() => load({ name: 'extensionZ' })}
        />
        <button
          data-testid="unloadExtensionZ"
          onClick={() => unload({ name: 'extensionZ' })}
        />
      </div>
    );
  };

  const renderTestComponent = () => render(
    <SWRConfig value={{ provider: () => new Map() }}>
      <TestComponent />
    </SWRConfig>
  );


  beforeEach(() => {
    global.fetch = fetch;
    vi.restoreAllMocks();
    cleanup();
  });

  describe('existing extensions', () => {
    it('should have existing extensions', async () => {
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => ([{
              name: 'extensionA',
            }, {
              name: 'extensionB',
            }
            ])
          } as unknown as Response;
        }
      });

      renderTestComponent();

      await waitFor(() => {
        expect(screen.getByTestId('ext-0')).toHaveTextContent('extensionA');
        expect(screen.getByTestId('ext-1')).toHaveTextContent('extensionB');
      });
    });
  });

  describe('loading', () => {
    it('should indicate loading only when loading', async () => {
      let resolveLoadingFn: () => void;
      const loadingPromise = new Promise<void>((resolve) => { resolveLoadingFn = resolve; });
      fetch.mockImplementation(async () => {
        await loadingPromise;

        return {
          ok: true,
          status: 200,
          json: async () => ([])
        } as unknown as Response;
      });

      renderTestComponent();

      await waitFor(async () => {
        expect(screen.getByTestId('loading')).toHaveTextContent('true');
      });

      await act(() => {
        resolveLoadingFn();
      });

      await waitFor(async () => {
        expect(screen.getByTestId('loading')).toHaveTextContent('false');
      });
    });
  });

  describe('error', () => {
    it('should show error on error', async () => {
      fetch.mockImplementation(async () => {
        return {
          ok: false,
          status: 400,
          text: async () => 'Some Error',
        } as unknown as Response;
      });

      renderTestComponent();

      await waitFor(() => {
        expect(screen.getByTestId('error')).toBeInTheDocument();
        expect(screen.getByTestId('error')).toHaveTextContent('Some Error');
      });
    });

    it('should not show error when there is no error', async () => {
      fetch.mockImplementation(async () => {
        return {
          ok: true,
          status: 200,
          json: async () => ([])
        } as unknown as Response;
      });

      renderTestComponent();

      await waitFor(() => {
        expect(screen.getByTestId('error')).toBeInTheDocument();
        expect(screen.getByTestId('error')).toBeEmptyDOMElement();
      });
    });
  });

  describe('upload', () => {
    it('should list new extension when uploaded', async () => {
      let hasUploaded = false;
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension/upload')) {
          hasUploaded = true;
          return {
            ok: true,
            status: 200,
            json: async () => ({
              name: 'extensionZ',
            })
          } as unknown as Response;
        }
        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => (hasUploaded ? [{
              name: 'extensionZ',
            }] : [])
          } as unknown as Response;
        }
      });
      renderTestComponent();

      await userEvent.click(screen.getByTestId('upload'));

      await waitFor(() => {
        expect(screen.getByTestId('ext-0')).toHaveTextContent('extensionZ');
      });
    });

    it('upload error are handled', async () => {
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension/upload')) {
          return {
            ok: false,
            status: 400,
            text: async () => 'Some upload error',
          } as unknown as Response;
        }
        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => ([])
          } as unknown as Response;
        }
      });
      renderTestComponent();

      await userEvent.click(screen.getByTestId('upload'));

      await waitFor(() => {
        expect(screen.getByTestId('error')).toHaveTextContent('Some upload error');
      });
    });
  });

  describe('remove', () => {
    it('should remove extension', async () => {
      let hasDelete = false;
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension/extensionZ')) {
          hasDelete = true;
          return {
            ok: true,
            status: 200,
            json: async () => ({
              name: 'extensionZ',
            })
          } as unknown as Response;
        }
        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => (hasDelete ? [] : [{
              name: 'extensionZ',
            }])
          } as unknown as Response;
        }
      });
      renderTestComponent();

      await userEvent.click(screen.getByTestId('removeExtensionZ'));

      await waitFor(() => {
        expect(screen.queryByTestId('ext-0')).not.toBeInTheDocument();
      });
    });
  });

  describe('load', () => {
    it('should load extension', async () => {
      let isLoaded = false;
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension/extensionZ/load')) {
          isLoaded = true;
          return {
            ok: true,
            status: 200,
            json: async () => ([{
              name: 'extensionZ',
              ipc_port: 123,
            }])
          } as unknown as Response;
        }

        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => ( isLoaded ? [{
              name: 'extensionZ',
              ipc_port: 123,
            }] : [{
              name: 'extensionZ',
            }])
          } as unknown as Response;
        }
      });
      renderTestComponent();

      await userEvent.click(screen.getByTestId('loadExtensionZ'));

      await waitFor(() => {
        expect(screen.queryByTestId('ext-0')).toBeInTheDocument();
        expect(screen.queryByTestId('ext-0')).toHaveTextContent('123');
      });
    });
  });

  describe('unload', () => {
    it('should unload extension', async () => {
      let isUnLoaded = false;
      fetch.mockImplementation(async (url) => {
        if (url.endsWith('/extension/extensionZ/unload')) {
          isUnLoaded = true;
          return {
            ok: true,
            status: 200,
            json: async () => ([{
              name: 'extensionZ',
              ipc_port: 123,
            }])
          } as unknown as Response;
        }

        if (url.endsWith('/extension')) {
          return {
            ok: true,
            status: 200,
            json: async () => ( isUnLoaded ? [{
              name: 'extensionZ',
            }] : [{
              name: 'extensionZ',
              ipc_port: 123,
            }])
          } as unknown as Response;
        }
      });
      renderTestComponent();

      await userEvent.click(screen.getByTestId('unloadExtensionZ'));

      await waitFor(() => {
        expect(screen.queryByTestId('ext-0')).toBeInTheDocument();
        expect(screen.queryByTestId('ext-0')).not.toHaveTextContent('123');
      });
    });
  });
});