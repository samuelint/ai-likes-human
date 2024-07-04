import type { Meta, StoryObj } from '@storybook/react';
import { ThreadPreview } from './thread-preview';


const meta = {
  title: 'Components/Thread Preview',
  component: ThreadPreview,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ThreadPreview>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    thread: {
      id: 'a',
      title: 'Hello, World!',
      created_at: new Date(),
    }
  },
};
