import type { Meta, StoryObj } from '@storybook/react';


import { ImageAttachment } from './image-attachment';
import { imageBase64 } from './image-asset.stories';




const meta = {
  title: 'Components/Image Attachment',
  component: ImageAttachment,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ImageAttachment>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    image: {
      title: 'Screenshot',
      base64: imageBase64,
    },
    onRemoveClick: () => {},
  },
};

export const SizeRestricted: Story = {
  args: {
    className: 'aspect-square',
    image: {
      title: 'Screenshot',
      base64: imageBase64,
    }
  },
};