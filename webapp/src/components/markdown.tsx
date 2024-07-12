
import LibMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { Prism } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/cjs/styles/prism';


interface Props {
  children?: string
}

export function Markdown({ children }: Props) {
  return (
    <LibMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        h1({ children }) {
          return <h1 className="text-4xl font-bold">{children}</h1>;
        },
        h2({ children }) {
          return <h2 className="text-3xl font-bold">{children}</h2>;
        },
        h3({ children }) {
          return <h3 className="text-2xl font-bold">{children}</h3>;
        },
        h4({ children }) {
          return <h4 className="text-xl font-bold">{children}</h4>;
        },
        h5({ children }) {
          return <h5 className="text-lg font-bold">{children}</h5>;
        },
        li({ children }) {
          return <li className="ml-4">{children}</li>;
        },
        ul({ children }) {
          return <ul className="list-disc list-inside">{children}</ul>;
        },
        ol({ children }) {
          return <ol className="list-decimal list-inside">{children}</ol>;
        },
        blockquote({ children }) {
          return <blockquote className="border-l-4 border-gray-300 pl-4">{children}</blockquote>;
        },
        code(props) {
          const { children, className, node, ...rest } = props;
          const match = /language-(\w+)/.exec(className || '');

          return match ? (
          // @ts-expect-error - Error from library
            <Prism
              {...rest}
              PreTag="div"
              children={String(children).replace(/\n$/, '')}
              language={match[1]}
              showLineNumbers
              lineNumberStyle={{ userSelect: 'none', MozUserSelect: 'none', WebkitUserSelect: 'none', msUserSelect: 'none' }}
              style={oneDark}
            />
          ) : (
            <code {...rest} className={className}>
              {children}
            </code>
          );
        }
      }}
    >
      {children}
    </LibMarkdown>
  );
}