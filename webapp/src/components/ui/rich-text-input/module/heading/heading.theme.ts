import { EditorThemeClasses } from 'lexical';


const heading: EditorThemeClasses['heading'] = {
  h1: 'pt-4 text-3xl font-extrabold dark:text-white',
  h2: 'pt-4 text-2xl font-bold dark:text-white',
  h3: 'pt-4 text-xl font-bold dark:text-white',
  h4: 'pt-4 text-lg font-bold dark:text-white',
  h5: 'pt-4 text-base font-bold dark:text-white',
};

const headingTheme: Required<Pick<EditorThemeClasses, 'heading'>> = { heading };

export { headingTheme };