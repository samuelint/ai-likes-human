import { MessageSquareShare } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';

export default function FeedbackButton() {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            type='button'
          >
            <MessageSquareShare className='w-4 h-4'/>
          </button>
        </TooltipTrigger>
        <TooltipContent>
          Give Feedback
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
