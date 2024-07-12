import type { Meta, StoryObj } from '@storybook/react';
import { Image } from './image';
import { imageBase64 } from './image-asset.stories';


const meta = {
  title: 'Components/Image',
  component: Image,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof Image>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    url: imageBase64,
  },
};
