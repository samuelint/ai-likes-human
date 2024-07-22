import type { Meta, StoryObj } from '@storybook/react';

import Splashscreen from './splashscreen';


const meta = {
  title: 'Components/Splashscreen',
  component: Splashscreen,
  parameters: {
    layout: 'centered',
    backgrounds: {
      default: 'light',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof Splashscreen>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
