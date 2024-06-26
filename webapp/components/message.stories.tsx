import type { Meta, StoryObj } from '@storybook/react';
import { Message } from './message';


const meta = {
  title: 'Components/Message',
  component: Message,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof Message>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    content: 'Hello',
  },
};

export const User: Story = {
  args: {
    content: 'Hello',
    type: 'user',
  },
};

export const AI: Story = {
  args: {
    content: 'Hello',
    type: 'ai',
  },
};

export const Actions: Story = {
  args: {
    content:
      `Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat
      Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
      `,
    type: 'user',
    actions: <>
      <button><CopyIcon/></button>
      <button><CopyIcon/></button>
      <button><CopyIcon/></button>
    </>
  },
};

export const MarkdownContent: Story = {
  args: {
    content: `
# h1
## h2
### h3
#### h4
##### h5
Paragraph
**bold**
*italic*

> Blockquote

- List item1
- List item2
- List item3

1. Ordered List item1
2. Ordered List item2
3. Ordered List item3

\`\`\`python
def my_function():
    pass;
\`\`\`
`,
    type: 'ai',
  },
};


function CopyIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="14"
      height="14"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
      <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
    </svg>
  );
}
