import type { Meta, StoryObj } from '@storybook/react';

import { ThreadsPreviewCollection } from './threads-preview-collection';


const meta = {
  title: 'Components/Threads Preview Collection',
  component: ThreadsPreviewCollection,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ThreadsPreviewCollection>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    isLoading: true,
    error: new Error('Something went wrong'),
    threads: [
      {
        id: '1',
        title: 'Hello, World!',
        created_at: new Date(),
      },
      {
        id: '2',
        title: 'Hello, World! 2',
        created_at: new Date(),
      },
      {
        id: '3',
        title: 'Hello, World! 3',
        created_at: new Date(),
      }
    ]
  },
};

export const Resolved: Story = {
  args: {
    threads: [
      {
        id: '1',
        title: 'Hello, World!',
        created_at: new Date(),
      },
      {
        id: '2',
        title: 'Hello, World! 2',
        created_at: new Date(),
      },
      {
        id: '3',
        title: 'Hello, World! 3',
        created_at: new Date(),
      }
    ]
  },
};

export const Loading: Story = {
  args: {
    isLoading: true
  },
};


export const Errored: Story = {
  args: {
    error: new Error('Something went wrong'),
  },
};
