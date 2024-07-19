import type { Meta, StoryObj } from '@storybook/react';
import { ExpandableImage } from './expandable-image';
import { imageBase64 } from './image-asset.stories';


const meta = {
  title: 'Components/ExpandableImage',
  component: ExpandableImage,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof ExpandableImage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    url: imageBase64,
  },
};
