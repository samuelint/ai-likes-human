import { EditorThemeClasses } from 'lexical';
import './text-format.css';


const text: EditorThemeClasses['text'] = {
  bold: 'font-bold',
  italic: 'italic',
  underline: 'underline',
  code: 'TextFormat__code',
};

const codeHighlight: EditorThemeClasses['codeHighlight'] = {
  atrule: 'text-blue-700',
  attr: 'text-blue-700',
  boolean: 'text-green-700',
  builtin: 'text-yellow-600',
  cdata: 'text-slategray',
  char: 'text-yellow-600',
  class: 'text-pink-600',
  'class-name': 'text-pink-600',
  comment: 'text-slategray',
  constant: 'text-green-700',
  deleted: 'text-green-700',
  doctype: 'text-slategray',
  entity: 'text-orange-600',
  function: 'text-red-600',
  important: 'text-purple-700',
  inserted: 'text-pink-600',
  keyword: 'text-blue-700',
  namespace: 'text-purple-700',
  number: 'text-green-700',
  operator: 'text-yellow-600',
  prolog: 'text-slategray',
  property: 'text-green-700',
  punctuation: 'text-gray-600',
  regex: 'text-purple-700',
  selector: 'text-pink-600',
  string: 'text-pink-600',
  symbol: 'text-green-700',
  tag: 'text-green-700',
  url: 'text-orange-600',
  variable: 'text-purple-700',
};

const quote: EditorThemeClasses['quote'] = 'mx-0 ml-5 mb-2 text-base text-gray-500 border-l-4 border-gray-300 pl-4';

const textFormatTheme: Required<Pick<EditorThemeClasses, 'text'| 'quote' | 'codeHighlight'>> = { text, quote, codeHighlight };

export { textFormatTheme };