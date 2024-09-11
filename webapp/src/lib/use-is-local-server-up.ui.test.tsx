/**
 * @jest-environment jsdom
 */

import '@testing-library/jest-dom';
import { cleanup, render, screen, waitFor, act } from '@testing-library/react';
import { useIsLocalServerUp } from './use-is-local-server-up';
import { isLocalServerRunning } from './tauri-command/server-status';
import { when } from 'jest-when';
import { useIsInDesktopAppFn } from './is-in-desktop-app';

vi.mock('./tauri-interrupt/server-status');
vi.mock('./is-in-desktop-app');
describe('new-conversation', () => {
  const TestComponent = () => {
    const { isUp, hasAlreadyBeenUp } = useIsLocalServerUp({ refreshInterval: 100 });

    return (
      <div>
        <div data-testid="isUp">{isUp ? 'true' : 'false'}</div>
        <div data-testid="hasAlreadyBeenUp">{hasAlreadyBeenUp ? 'true' : 'false'}</div>
      </div>
    );
  };

  beforeEach(() => {
    when(useIsInDesktopAppFn).mockImplementation(() => () => true);
  });

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });


  describe('is up', () => {
    it('should be up when is up at mount', async () => {
      when(isLocalServerRunning).mockResolvedValue(true);

      render(<TestComponent />);

      await waitFor(async () => {
        await screen.findByTestId('isUp');
        expect(screen.getByTestId('isUp')).toHaveTextContent(
          'true',
        );
      });

    });

    it('should be up when is up after a while', async () => {
      when(isLocalServerRunning).mockImplementation(async () => new Promise(resolve => setTimeout(() => resolve(true), 200)));

      render(<TestComponent />);

      await new Promise(resolve => setTimeout(resolve, 200));
      await waitFor(async () => {
        await screen.findByTestId('isUp');
        expect(screen.getByTestId('isUp')).toHaveTextContent(
          'true',
        );
      });
    });

    it('should not be up when never set to up', async () => {
      when(isLocalServerRunning).mockImplementation(async () => new Promise(resolve => setTimeout(() => resolve(false), 200)));

      render(<TestComponent />);

      await new Promise(resolve => setTimeout(resolve, 200));
      await waitFor(async () => {
        await screen.findByTestId('isUp');
        expect(screen.getByTestId('isUp')).toHaveTextContent(
          'false',
        );
      });
    });

    it('should be down when is down after being up', async () => {
      when(isLocalServerRunning).mockResolvedValue(true);

      render(<TestComponent />);

      await act(async () => {
        await new Promise(resolve => setTimeout(resolve, 150));
        when(isLocalServerRunning).mockResolvedValue(false);
        await new Promise(resolve => setTimeout(resolve, 100));
      });


      await waitFor(async () => {
        await screen.findByTestId('isUp');
        expect(screen.getByTestId('isUp')).toHaveTextContent(
          'false',
        );
      });
    });
  });

  describe('is already been up', () => {

    it('should be already been up when up', async () => {
      when(isLocalServerRunning).mockResolvedValue(true);

      render(<TestComponent />);

      await waitFor(async () => {
        await screen.findByTestId('hasAlreadyBeenUp');
        expect(screen.getByTestId('hasAlreadyBeenUp')).toHaveTextContent(
          'true',
        );
      });
    });

    it('should be already been up when been up at least one time', async () => {
      when(isLocalServerRunning).mockResolvedValue(true);

      render(<TestComponent />);

      await act(async () => {
        await new Promise(resolve => setTimeout(resolve, 150));
        when(isLocalServerRunning).mockResolvedValue(false);
        await new Promise(resolve => setTimeout(resolve, 100));
      });

      await waitFor(async () => {
        await screen.findByTestId('hasAlreadyBeenUp');
        expect(screen.getByTestId('hasAlreadyBeenUp')).toHaveTextContent(
          'true',
        );
      });
    });
  });
});
