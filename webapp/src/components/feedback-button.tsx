import { MessageSquareShare } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';

export default function FeedbackButton() {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <MessageSquareShare className='w-4 h-4'/>
        </TooltipTrigger>
        <TooltipContent>
          Give Feedback
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
