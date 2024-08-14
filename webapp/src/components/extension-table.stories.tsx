import type { Meta, StoryObj } from '@storybook/react';
import { ExtensionTable } from './extension-table';
import { Button } from './ui/button';



const meta = {
  title: 'Components/Extension/Table',
  component: ExtensionTable,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ExtensionTable>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    extensions: [
      {
        name: 'A',
        ipc_port: 123,
        status: 'installed',
      },
      {
        name: 'B',
        ipc_port: 543,
        status: 'loaded',
      }
    ],
    Actions: (extension) => <>
      <Button>Load {extension.name}</Button>
      <Button variant="destructive">Delete</Button>
    </>
  }
};
