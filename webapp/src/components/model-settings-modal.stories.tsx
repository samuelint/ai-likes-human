import type { Meta, StoryObj } from '@storybook/react';
import { ModelSelectorModal } from './model-settings-modal';
import { Button } from './ui/button';


const meta = {
  title: 'Components/Model Settings Modal',
  component: ModelSelectorModal,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
    onSubmit: { action: 'onSubmit' },
  },
} satisfies Meta<typeof ModelSelectorModal>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    choices: {
      models: ['openai:gpt-4o', 'anthropic:sonnet-3.5', 'local:llama3', 'local:phi3']
    },
    children: <Button variant="outline">Open</Button>
  },
};
