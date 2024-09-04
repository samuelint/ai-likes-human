import type { Meta, StoryObj } from '@storybook/react';
import { SecretInput } from './secret.input';


const meta = {
  title: 'Components/Secret Input',
  component: SecretInput,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    onSave: { action: 'onSave' },
    onChange: { action: 'onChange' },
    disabled: { type: 'boolean' },
    label: { type: 'string' }
  },
} satisfies Meta<typeof SecretInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'Some Secret',
    saveLabel: 'Save',
  }
};

export const WithoutSave: Story = {
  args: {
    label: 'Some Secret',
    defaultValue: '1234'
  }
};

export const InitiallyDefined: Story = {
  args: {
    label: 'Some Secret',
    saveLabel: 'Save',
    defaultValue: '12341234123412341234123412341234123412341234'
  }
};

export const Footer: Story = {
  args: {
    label: 'Some Secret',
    saveLabel: 'Save',
    defaultValue: '12341234123412341234123412341234123412341234',
    footer: <span>Some Footer</span>
  }
};

export const Children: Story = {
  args: {
    label: 'Some Secret',
    saveLabel: 'Save',
    defaultValue: '12341234123412341234123412341234123412341234',
    children: <span>Some Child</span>
  }
};