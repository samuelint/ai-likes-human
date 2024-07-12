import { EditorThemeClasses } from 'lexical';
import './list.css';


const listTheme: EditorThemeClasses = {
  list: {
    listitem: 'ListTheme__listItem',
    nested: {
      listitem: 'ListTheme__nestedListItem',
    },
    olDepth: [
      'ListTheme__ol1',
      'ListTheme__ol2',
      'ListTheme__ol3',
      'ListTheme__ol4',
      'ListTheme__ol5',
    ],
    ol: 'ListTheme__ol',
    ul: 'ListTheme__ul',
  }
};

export { listTheme };