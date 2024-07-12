import { ButtonHTMLAttributes } from 'react';
import { twMerge } from 'tailwind-merge';


interface Props extends ButtonHTMLAttributes<HTMLButtonElement> { }

export function ToolbarButton({ className, children, ...props }: Props) {

  return (
    <button
      {...props}
      className={twMerge('icon-button disabled:cursor-not-allowed', className)}
    >
      { children }
    </button>
  );
}