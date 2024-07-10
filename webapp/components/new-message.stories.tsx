import type { Meta, StoryObj } from '@storybook/react';
import NewMessage from './new-message';
import { Button } from './ui/button';
import { LaptopMinimal } from 'lucide-react';
import { Card } from './ui/card';


const meta = {
  title: 'Components/New Message',
  component: NewMessage,
  parameters: {
    layout: 'padded',
    backgrounds: {
      default: 'white',
    },
  },
  tags: ['autodocs'],
  argTypes: {
  },
} satisfies Meta<typeof NewMessage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    placeholder: 'Type your message...',
    input: '',
    disabled: false,
    isLoading: false,
  },
};

export const Children: Story = {
  args: {
    placeholder: 'Type your message...',
    input: '',
    disabled: false,
    isLoading: false,
    leftContent: <Button className='p-1' variant='ghost'><LaptopMinimal/></Button>,
    rightContent: <span className='text-xs'>Right Content</span>,
    children: [<Card key={0}>Attachments 1</Card>, <Card key={1}>Attachments 2</Card>],
  },
};


