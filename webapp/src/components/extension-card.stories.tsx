import type { Meta, StoryObj } from '@storybook/react';
import { ExtensionCard } from './extension-card';
import { Button } from './ui/button';


const meta = {
  title: 'Components/Extension/Card',
  component: ExtensionCard,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ExtensionCard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    extension: {
      name: 'My Extension',
      description: 'My extension description',
      author: 'Me',
      version: '0.1.0',
    }
  }
};

export const Actions: Story = {
  args: {
    extension: {
      name: 'My Extension',
      description: 'My extension description',
      author: 'Me',
      version: '0.1.0',
    },
    children: (<>
      <Button variant="destructive">Delete</Button>
      <Button variant="outline">Reset</Button>
    </>)
  }
};
