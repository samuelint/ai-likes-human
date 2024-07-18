import type { Meta, StoryObj } from '@storybook/react';
import RichTextInput from './rich-text-input';


const meta = {
  title: 'Components/Rich Text Input',
  component: RichTextInput,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    editable: { type: 'boolean' },
    onChange: { action: 'onChange' }
  },
} satisfies Meta<typeof RichTextInput>;

export default meta;
type Story = StoryObj<typeof meta>;


export const Default: Story = {
  args: {
    input: '',
    debug: true,
  },
};


export const Primary: Story = {
  args: {
    input: `
# Title
## Subtitle
    
This is markdown content.

https://google.com

* Bullet 1
    * Nested
1. Ordered 1
    1. Nested
2. Ordered 2

*Italic*
**Bold**
\`code\`


\`\`\`typescript
const c = 'this is large code block';
\`\`\`
    `
  },
};

export const Bug__ListAndCode: Story = {
  args: {
    input: `To debounce a \`useState\` setter in a React component using TypeScript, you can leverage the \`useEffect\` and \`useCallback\` hooks along with \`setTimeout\`. Here’s how you can implement a debounced setter:

    1. **Install Lodash** (optional, but recommended for debouncing):
       
       You can use Lodash's \`debounce\` function for cleaner and more reliable debouncing. If you're not using Lodash, you'll manually implement debouncing logic.
    
       \`\`\`bash
       npm install lodash
       \`\`\`
    
    2. **Example Component with Debounced State Setter**:
    
       \`\`\`tsx
       import React, { useState, useEffect, useCallback } from 'react';
       import { debounce } from 'lodash';
    
       const DebouncedInput: React.FC = () => {
         // Regular state
         const [inputValue, setInputValue] = useState('');
         // State to hold the debounced value
         const [debouncedValue, setDebouncedValue] = useState('');
    
         // Debounce the setter
         const debouncedSetInputValue = useCallback(
           debounce((nextValue: string) => setDebouncedValue(nextValue), 500),
           [],
         );
    
         // Effect to handle input changes with debouncing
         useEffect(() => {
           debouncedSetInputValue(inputValue);
           // Cancel the debounce on useEffect cleanup
           return () => {
             debouncedSetInputValue.cancel();
           };
         }, [inputValue, debouncedSetInputValue]);
    
         return (
           <div>
             <input
               type="text"
               onChange={(e) => setInputValue(e.target.value)}
               value={inputValue}
             />
             <p>Debounced Value: {debouncedValue}</p>
           </div>
         );
       };
    
       export default DebouncedInput;
       \`\`\`
    
    In this setup, \`setInputValue\` updates the \`inputValue\` state as usual, but the \`debouncedValue\` is only updated after the specified delay (500 ms in this example) using the \`debouncedSetInputValue\` function. This approach ensures that the debounced state (\`debouncedValue\`) is only updated after the user has stopped typing for a specified duration, thereby reducing the number of updates.`
  },
};
