import { ChangeEvent } from 'react';


export function createSyntheticChangeEvent<T = Element>(value: string): ChangeEvent<T> {
  return {
    // @ts-expect-error - Value is a string
    target: {
      value,
    },
  };
}
