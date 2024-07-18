import { SerializedEditorState } from 'lexical';
import { InitialConfigType, LexicalComposer } from '@lexical/react/LexicalComposer';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import { AutoFocusPlugin } from '@lexical/react/LexicalAutoFocusPlugin';
import { HistoryPlugin } from './module/history';
import LexicalErrorBoundary from '@lexical/react/LexicalErrorBoundary';
import { textFormatTheme } from './module/text-format';
import { ChangeEvent, PropsWithChildren } from 'react';
import { HeadingNodes, HeadingPlugin } from './module/heading';
import { headingTheme } from './module/heading/heading.theme';
import { MarkdownNodes, MarkdownPlugin, markdownTheme } from './module/markdown';
import { TextFormatNodes, TextFormatPlugin } from './module/text-format/text-format.plugin';
import { CodeHighlightNodes, CodeHighlightPlugin, codeHighlightTheme } from './module/code-highlight';
import { OnMarkdownChangePlugin } from './module/on-change';
import { ListNodes, ListPlugin, listTheme } from './module/list';
import { LinkNodes, LinkPlugin, linkTheme } from './module/link';
import { ExposeKeyboardEvent } from './module/keyboard-events';
import { cn } from '@/lib/utils';
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin';

export type InputState = SerializedEditorState;
export type OnError = (error: Error) => void;

export type KeyboarEvents = 'Submit';

interface Props extends PropsWithChildren {
  input?: string
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => void
  placeholder?: string

  onError?: OnError
  disabled?: boolean
  editable?: boolean
  className?: string
  onKeyboardEvent?: (event: KeyboarEvents) => void
  debug?: boolean
}

export default function RichTextInput({ input, placeholder, onChange, onError, disabled, editable = true, className, onKeyboardEvent, debug, children }: Props) {
  const initialConfig: InitialConfigType = {
    nodes: [...HeadingNodes, ...ListNodes, ...MarkdownNodes, ...TextFormatNodes, ...CodeHighlightNodes, ...LinkNodes],
    editable,
    namespace: 'Conversation',
    theme: {
      ...listTheme,
      ...headingTheme,
      ...textFormatTheme,
      ...markdownTheme,
      ...codeHighlightTheme,
      ...linkTheme,
      rtl: 'block',
      ltr: 'block',
      paragraph: 'block'
    },
    onError: (e) => onError && onError(e),
  };

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <RichTextPlugin
        contentEditable={<ContentEditable disabled={disabled} className={cn('w-full rounded p-2 bg-white', className)} />}
        placeholder={<div className='absolute left-0 pointer-events-none text-gray-400'>{placeholder}</div>}
        ErrorBoundary={LexicalErrorBoundary}
      />
      { debug && <OnChangePlugin onChange={(editorState) => console.log(JSON.stringify(editorState))} />}

      <HistoryPlugin />
      <ExposeKeyboardEvent
        onSendEvent={() => onKeyboardEvent && onKeyboardEvent('Submit')}
        isKeyboardEvent={(event) => (event.metaKey || event.ctrlKey) && event.key === 'Enter'}/>
      <TextFormatPlugin />
      <CodeHighlightPlugin />
      <HeadingPlugin />
      <ListPlugin />
      <LinkPlugin />
      <MarkdownPlugin />
      <AutoFocusPlugin defaultSelection='rootEnd' />
      <OnMarkdownChangePlugin
        input={input}
        onChange={onChange}
      />
      { children }
    </LexicalComposer>
  );
}

