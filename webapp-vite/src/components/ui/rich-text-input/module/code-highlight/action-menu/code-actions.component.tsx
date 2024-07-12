/*
 * This code is not SOLID. It's a almost copy/paste of the official example.
 * https://github.com/facebook/lexical/blob/main/packages/lexical-playground/src/plugins/CodeActionMenuPlugin/index.tsx
 */
import './code-actions.css';

import {
  $isCodeNode,
  CodeNode,
  getLanguageFriendlyName,
  normalizeCodeLang,
} from '@lexical/code';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getNearestNodeFromDOMNode } from 'lexical';
import { useEffect, useRef, useState } from 'react';
import * as React from 'react';
import { useDebounce } from '../../../utils/use-debounce';




const CODE_PADDING = 8;

interface Position {
  top: string;
  right: string;
}

interface ChildrenProps {
  lang: string
  normalizedLang: string
  codeFriendlyName: string

  getCodeDOMNode: () => HTMLElement | null
}

export type CodeActionsChildren = (props: ChildrenProps) => React.ReactNode;

interface Props {
  anchorElem: HTMLElement;
  children?: CodeActionsChildren
}

export function CodeActions({
  anchorElem,
  children,
}: Props): JSX.Element {
  const [editor] = useLexicalComposerContext();

  const [lang, setLang] = useState('');
  const [isShown, setShown] = useState<boolean>(false);
  const [shouldListenMouseMove, setShouldListenMouseMove] =
    useState<boolean>(false);
  const [position, setPosition] = useState<Position>({
    right: '0',
    top: '0',
  });
  const codeSetRef = useRef<Set<string>>(new Set());
  const codeDOMNodeRef = useRef<HTMLElement | null>(null);

  function getCodeDOMNode(): HTMLElement | null {
    return codeDOMNodeRef.current;
  }

  const debouncedOnMouseMove = useDebounce(
    (event: MouseEvent) => {
      const { codeDOMNode, isOutside } = getMouseInfo(event);
      if (isOutside) {
        setShown(false);
        return;
      }

      if (!codeDOMNode) {
        return;
      }

      codeDOMNodeRef.current = codeDOMNode;

      let codeNode: CodeNode | null = null;
      let _lang = '';

      editor.update(() => {
        const maybeCodeNode = $getNearestNodeFromDOMNode(codeDOMNode);

        if ($isCodeNode(maybeCodeNode)) {
          codeNode = maybeCodeNode;
          _lang = codeNode.getLanguage() || '';
        }
      });

      if (codeNode) {
        const { y: editorElemY, right: editorElemRight } =
          anchorElem.getBoundingClientRect();
        const { y, right } = codeDOMNode.getBoundingClientRect();
        setLang(_lang);
        setShown(true);
        setPosition({
          right: `${editorElemRight - right + CODE_PADDING}px`,
          top: `${y - editorElemY}px`,
        });
      }
    },
    50,
    1000,
  );

  useEffect(() => {
    if (!shouldListenMouseMove) {
      return;
    }

    document.addEventListener('mousemove', debouncedOnMouseMove);

    return () => {
      setShown(false);
      debouncedOnMouseMove.cancel();
      document.removeEventListener('mousemove', debouncedOnMouseMove);
    };
  }, [shouldListenMouseMove, debouncedOnMouseMove]);

  editor.registerMutationListener(CodeNode, (mutations) => {
    editor.getEditorState().read(() => {
      for (const [key, type] of mutations) {
        switch (type) {
        case 'created':
        case 'updated':
          codeSetRef.current.add(key);
          setShouldListenMouseMove(codeSetRef.current.size > 0);
          break;

        case 'destroyed':
          codeSetRef.current.delete(key);
          setShouldListenMouseMove(codeSetRef.current.size > 0);
          break;

        default:
          break;
        }
      }
    });
  });
  const normalizedLang = normalizeCodeLang(lang);
  const codeFriendlyName = getLanguageFriendlyName(lang);

  return (
    <>
      {isShown ? (
        <div id="code-action-menu" className="code-action-menu-container rounded-lg bg-slate-600/50 backdrop-blur-sm px-2 py-1 mt-2 text-slate-200" style={{ ...position }}>
          { children && children({ lang, normalizedLang, codeFriendlyName, getCodeDOMNode })}
        </div>
      ) : null}
    </>
  );
}

function getMouseInfo(event: MouseEvent): {
  codeDOMNode: HTMLElement | null;
  isOutside: boolean;
} {
  const target = event.target;

  if (target && target instanceof HTMLElement) {
    const codeDOMNode = target.closest<HTMLElement>(
      'code',
    );
    const isOutside = !(
      codeDOMNode ||
      target.closest<HTMLElement>('#code-action-menu')
    );

    return { codeDOMNode, isOutside };
  } else {
    return { codeDOMNode: null, isOutside: true };
  }
}

