import type { Meta, StoryObj } from '@storybook/react';
import { AddLocalExtensionForm } from './add-local-extension-form';


const meta = {
  title: 'Components/Extension/Add Local',
  component: AddLocalExtensionForm,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
    onSubmit: {
      action: 'onSubmit',
    }
  },
} satisfies Meta<typeof AddLocalExtensionForm>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    onSubmit: (event) => { event.preventDefault; },
  }
};
