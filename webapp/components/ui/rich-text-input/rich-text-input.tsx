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
import { BannerNodes, BannerPlugin, bannerTheme } from './module/banner';
import { TextFormatNodes, TextFormatPlugin } from './module/text-format/text-format.plugin';
import { CodeHighlightNodes, CodeHighlightPlugin, codeHighlightTheme } from './module/code-highlight';
import { _TRANSFORMERS } from './module/markdown/transformers';
import { OnMarkdownChangePlugin } from './module/on-change';
import { ListNodes, ListPlugin, listTheme } from './module/list';
import { LinkNodes, LinkPlugin, linkTheme } from './module/link';
import { ExposeKeyboardEvent } from './module/keyboard-events';
import { cn } from '@/lib/utils';


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
}

export default function RichTextInput({ input, placeholder, onChange, onError, disabled, editable = true, className, onKeyboardEvent, children }: Props) {
  const initialConfig: InitialConfigType = {
    nodes: [...HeadingNodes, ...ListNodes, ...MarkdownNodes, ...BannerNodes, ...TextFormatNodes, ...CodeHighlightNodes, ...LinkNodes],
    editable,
    namespace: 'Conversation',
    theme: {
      ...listTheme,
      ...bannerTheme,
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
        placeholder={<div className='absolute left-6 pointer-events-none text-gray-400'>{placeholder}</div>}
        ErrorBoundary={LexicalErrorBoundary}
      />

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
      <BannerPlugin />
      <AutoFocusPlugin defaultSelection='rootEnd' />
      <OnMarkdownChangePlugin
        input={input}
        onChange={onChange}
      />
      { children }
    </LexicalComposer>
  );
}

