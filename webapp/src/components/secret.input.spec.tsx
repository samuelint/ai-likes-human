/**
 * @jest-environment jsdom
 */
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import { SecretInput } from './secret.input';
import { act } from 'react-dom/test-utils';


function getInput(): HTMLElement {
  return screen.getByRole('textbox');
}

async function getVisibilityButton(): Promise<HTMLElement> {
  return (await screen.findAllByRole('button'))[0];
}
async function clickVisibilityButton(): Promise<void> {
  const visibilityButton = await getVisibilityButton();
  await visibilityButton.click();
}

test(`given no initial value,
      when rendering,
      then save button is disabled`, async () => {
  render(<SecretInput saveLabel='Save'/>);

  expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
});

test(`given no initial value,
      when typing new value,
      then save button is enabled`, async () => {
  render(<SecretInput saveLabel='Save' />);

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'hello');
  });

  await waitFor(() => {
    expect(screen.getByRole('button', { name: 'Save' })).toBeEnabled();
  });
});

test(`given initial value,
      when typing new value,
        and pressing save button,
      then save callback is called`, async () => {
  const onSave = jest.fn();
  render(<SecretInput saveLabel='Save' defaultValue='some' onSave={onSave} />);

  await act(async () => {
    await clickVisibilityButton();
  });

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'hello');
  });

  screen.getByRole('button', { name: 'Save' }).click();

  await waitFor(() => {
    expect(onSave).toHaveBeenCalledWith('somehello');
  });
});

test(`given initial value,
      when rendering,
      then save button is disabled`, async () => {
  render(<SecretInput saveLabel='Save' defaultValue='some'/>);

  expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
});

test(`given initial value,
      when setting input visible
        and changing the value,
      then save button is enabled`, async () => {
  render(<SecretInput saveLabel='Save' defaultValue='some'/>);

  await act(async () => {
    await clickVisibilityButton();
  });

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'newChars');
  });

  await waitFor(() => {
    expect(screen.getByRole('button', { name: 'Save' })).toBeEnabled();
  });
});

test(`given initial value,
      when setting input visible,
        changing value,
        and setting it back to initial,
      then save button is disabled`, async () => {
  render(<SecretInput saveLabel='Save' defaultValue='some'/>);

  await act(async () => {
    await clickVisibilityButton();
  });

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'newChars');
    await userEvent.clear(input);
    await userEvent.type(input, 'some');
  });

  await waitFor(() => {
    expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
  });
});

test(`given initial value,
      when editing value and new initial value is set,
      then save button is disabled`, async () => {
  const { rerender } = render(<SecretInput saveLabel='Save' defaultValue='AAA'/>);

  await act(async () => {
    await clickVisibilityButton();
  });

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'newChars');
  });

  rerender(<SecretInput saveLabel='Save' defaultValue='BBB'/>);

  expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
});

test(`given initial value,
      when enabled characters visibility, editing value and new initial value is set,
      then characters are obfuscated`, async () => {
  const { rerender } = render(<SecretInput saveLabel='Save' defaultValue='AAA'/>);

  await act(async () => {
    await clickVisibilityButton();
  });

  await act(async () => {
    const input = getInput();
    await userEvent.click(input);
    await userEvent.type(input, 'newChars');
  });

  rerender(<SecretInput saveLabel='Save' defaultValue='BBB'/>);

  const visibilityButton = await getVisibilityButton();
  expect(visibilityButton).toHaveAttribute('data-show-characters');
});