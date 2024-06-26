import { preprocessMarkdown } from './markdown.loader';


describe('preprocessMarkdown', () => {
  it(`given content with un even triple backtick,
      when pre-processing markdown,
      then there's an even triple backtick group`, () => {
    const content = preprocessMarkdown('```typescript\nyo');

    expect(content).toEqual('```typescript\nyo\n```');
  });
});