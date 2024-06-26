export function preprocessMarkdown(content: string): string {
  return pipeStringFunctions(content, [appendBacktickIfCodeBlockIsOpen]);
}

function appendBacktickIfCodeBlockIsOpen(content: string): string {
  const backtickCount = content.split('```').length - 1;
  const isEven = backtickCount % 2 == 0;

  if (!isEven) {
    content += '\n```';
  }

  return content;
}

const pipeStringFunctions = (input: string, functions: ((str: string) => string)[]): string => {
  return functions.reduce((acc, func) => func(acc), input);
};
