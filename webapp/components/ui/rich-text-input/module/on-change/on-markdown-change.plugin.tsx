import { OnChangePlugin as LexicalOnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';
import { $convertFromMarkdownString, $convertToMarkdownString } from '@lexical/markdown';
import { _TRANSFORMERS } from '../markdown/transformers';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { ChangeEvent, useEffect, useRef } from 'react';
import { preprocessMarkdown } from '../markdown/markdown.loader';
import { createSyntheticChangeEvent } from '../../utils/synthetic-on-change-event';


interface Props {
  input?: string
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => void
}

export function OnMarkdownChangePlugin({ input, onChange }: Props) {
  const [editor] = useLexicalComposerContext();
  const changedValueRef = useRef('');

  useEffect(() => {
    editor.update(() => {
      if (input == null) return;
      const current = $convertToMarkdownString(_TRANSFORMERS);
      if (current === input) return;

      $convertFromMarkdownString(preprocessMarkdown(input), _TRANSFORMERS);
    });
  }, [editor, input]);

  return <LexicalOnChangePlugin onChange={() => {
    if (onChange) {
      editor.update(() => {
        const newValue = $convertToMarkdownString(_TRANSFORMERS);
        if (newValue === input || changedValueRef.current === newValue) return;
        changedValueRef.current = newValue;
        onChange(createSyntheticChangeEvent<HTMLTextAreaElement>(newValue));
      });
    }
  }} />;
}
