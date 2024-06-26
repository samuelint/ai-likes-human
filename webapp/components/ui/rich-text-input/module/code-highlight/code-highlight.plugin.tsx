import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { useEffect } from 'react';
import { CodeHighlightNode, CodeNode, registerCodeHighlighting } from '@lexical/code';
import { FORMAT_CODE_COMMAND, formatCode } from './format-code.command';
import { COMMAND_PRIORITY_NORMAL } from 'lexical';
import 'prismjs/components/prism-json';
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-applescript';
import 'prismjs/components/prism-asciidoc';
import 'prismjs/components/prism-aspnet';
import 'prismjs/components/prism-basic';
import 'prismjs/components/prism-brainfuck';
import 'prismjs/components/prism-csharp';
import 'prismjs/components/prism-clojure';
import 'prismjs/components/prism-cmake';
import 'prismjs/components/prism-cobol';
import 'prismjs/components/prism-coffeescript';
import 'prismjs/components/prism-csp';
import 'prismjs/components/prism-csv';
import 'prismjs/components/prism-docker';
import 'prismjs/components/prism-dot';
import 'prismjs/components/prism-erlang';
import 'prismjs/components/prism-gcode';
import 'prismjs/components/prism-go';
import 'prismjs/components/prism-go-module';
import 'prismjs/components/prism-gradle';
import 'prismjs/components/prism-graphql';
import 'prismjs/components/prism-latex';
import 'prismjs/components/prism-lisp';
import 'prismjs/components/prism-makefile';
import 'prismjs/components/prism-mongodb';
import 'prismjs/components/prism-perl';
import 'prismjs/components/prism-php';
import 'prismjs/components/prism-jsx';
import 'prismjs/components/prism-tsx';
import 'prismjs/components/prism-ruby';
import 'prismjs/components/prism-swift';
import 'prismjs/components/prism-unrealscript';
import 'prismjs/components/prism-wasm';
import 'prismjs/components/prism-yaml';


const CodeHighlightNodes = [
  CodeHighlightNode,
  CodeNode,
];
export { CodeHighlightNodes };


export function CodeHighlightPlugin() {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return registerCodeHighlighting(editor);
  }, [editor]);

  useEffect(() => {
    return editor.registerCommand(
      FORMAT_CODE_COMMAND,
      formatCode,
      COMMAND_PRIORITY_NORMAL,
    );
  }, [editor]);

  return (<></>);
}
