import { UseExposeKeyboardEventProps, useExposeKeyboardEvent } from './use-expose-keyboard-event';


export type ExposeKeyboardEventProps = UseExposeKeyboardEventProps;

export function ExposeKeyboardEvent(props: ExposeKeyboardEventProps) {
  useExposeKeyboardEvent(props);

  return (<></>);
}
