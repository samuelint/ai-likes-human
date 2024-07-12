import type { Meta, StoryObj } from '@storybook/react';
import { ModelSettingsForm } from './model-settings-form';


const meta = {
  title: 'Components/Model Settings Form',
  component: ModelSettingsForm,
  parameters: {
    layout: 'padded',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
    onSubmit: { action: 'onSubmit' },
  },
} satisfies Meta<typeof ModelSettingsForm>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    defaultValues: {
      model: 'openai:gpt-4o'
    },
    choices: {
      models: ['openai:gpt-4o', 'anthropic:sonnet-3.5', 'local:llama3', 'local:phi3']
    },
  },
};
