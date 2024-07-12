import { EditorThemeClasses } from 'lexical';
import './code.css';


const codeHighlight: EditorThemeClasses['codeHighlight'] = {
  atrule: 'CodeHighlight__tokenAttr',
  attr: 'CodeHighlight__tokenAttr',
  boolean: 'CodeHighlight__tokenProperty',
  builtin: 'CodeHighlight__tokenSelector',
  cdata: 'CodeHighlight__tokenComment',
  char: 'CodeHighlight__tokenSelector',
  class: 'CodeHighlight__tokenFunction',
  'class-name': 'CodeHighlight__tokenFunction',
  comment: 'CodeHighlight__tokenComment',
  constant: 'CodeHighlight__tokenProperty',
  deleted: 'CodeHighlight__tokenProperty',
  doctype: 'CodeHighlight__tokenComment',
  entity: 'CodeHighlight__tokenOperator',
  function: 'CodeHighlight__tokenFunction',
  important: 'CodeHighlight__tokenVariable',
  inserted: 'CodeHighlight__tokenSelector',
  keyword: 'CodeHighlight__tokenAttr',
  namespace: 'CodeHighlight__tokenVariable',
  number: 'CodeHighlight__tokenProperty',
  operator: 'CodeHighlight__tokenOperator',
  prolog: 'CodeHighlight__tokenComment',
  property: 'CodeHighlight__tokenProperty',
  punctuation: 'CodeHighlight__tokenPunctuation',
  regex: 'CodeHighlight__tokenVariable',
  selector: 'CodeHighlight__tokenSelector',
  string: 'CodeHighlight__tokenSelector',
  symbol: 'CodeHighlight__tokenProperty',
  tag: 'CodeHighlight__tokenProperty',
  url: 'CodeHighlight__tokenOperator',
  variable: 'CodeHighlight__tokenVariable',
};
const code: EditorThemeClasses['code'] = 'CodeHighlight__code';

const codeHighlightTheme: Required<Pick<EditorThemeClasses, 'code' | 'codeHighlight'>> = { code, codeHighlight };

export { codeHighlightTheme };