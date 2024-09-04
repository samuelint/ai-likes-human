'use client';
import { ChangeEventHandler, MouseEventHandler, ReactNode, useEffect, useState } from 'react';
import { HideIcon } from './icon/hide.icon';
import { ShowIcon } from './icon/show.icon';
import { Input } from './ui/input';
import { Button } from './ui/button';



export type OnSave = (key: string) => void;
export type OnChange = (key: string) => void;
interface Props {
  label?: string
  id?: string
  name?: string
  defaultValue?: string | null
  placeholder?: string
  footer?: ReactNode

  onChange?: OnChange
  onSave?: OnSave
  saveLabel?: string

  className?: string
  disabled?: boolean

  children?: ReactNode
}

export function SecretInput({ label, id, name, defaultValue, placeholder, className, disabled, footer, saveLabel, onSave, onChange, children }: Props) {
  const [showCharacters, setShowCharacters] = useState(!defaultValue || defaultValue.length <= 0);
  const [value, setValue] = useState(defaultValue ?? '');
  const [isSaveButtonEnabled, setIsSaveButtonEnabled] = useState(false);

  const toggleCharactersVisibility = () => {
    setShowCharacters(!showCharacters);
  };

  useEffect(() => {
    const newInitialValue = defaultValue ?? '';
    setValue(newInitialValue);
    setShowCharacters(!newInitialValue || newInitialValue.length <= 0);
    setIsSaveButtonEnabled(false);
  }, [defaultValue]);

  const onChangeInternal: ChangeEventHandler<HTMLInputElement> = (event) => {
    const newValue = event.target.value;
    setValue(newValue);
    setIsSaveButtonEnabled(newValue.length > 0 && newValue !== defaultValue);
    onChange && onChange(newValue);
  };

  const onSaveClick: MouseEventHandler = () => {
    if (value !== defaultValue && onSave) {
      onSave(value);
    }
  };

  return (
    <div>
      <label htmlFor={`#${id}`} className='text-gray-600 capitalize'>{label}</label>
      <div className={`w-full flex items-center justify-center gap-1 min-w-72 ${className}`}>
        <div className="w-full relative flex items-center">
          <Input
            value={value}
            onChange={onChangeInternal}
            id={id}
            name={name}
            type={showCharacters ? 'text' : 'password'}
            autoComplete="off"
            readOnly
            disabled={disabled}
            onKeyDown={(e) => {
              if (!showCharacters) e.preventDefault();
            }}
            onFocus={(e) => e.target.removeAttribute('readonly')}
            placeholder={placeholder}
            className='pr-10'
          />
          <div className='absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5 gap-1'>
            <button
              type="button"
              id='visibility'
              disabled={disabled}
              data-show-characters={showCharacters}
              onClick={toggleCharactersVisibility}
              className="text-gray-400"
            >
              {showCharacters ? (
                <HideIcon className='w-4 h-4' />
              ) : (
                <ShowIcon className='w-4 h-4'/>
              )}
            </button>
          </div>
        </div>
        { saveLabel &&
              <Button
                className='inline-flex justify-center rounded-md border-transparent text-sm font-medium px-2 py-1'
                disabled={!isSaveButtonEnabled}
                onClick={onSaveClick}
              >
                { saveLabel }
              </Button>
        }
        { children }
      </div>
      { footer && <label htmlFor={`#${id}`} className='text-gray-300 text-xs'>{footer}</label> }
    </div>
  );
}
