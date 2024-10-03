import { useEffect, useRef, useState } from 'react';
import * as Sentry from '@sentry/react';
import { MessageSquareShare } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';

export default function FeedbackButton() {
  // Sentry types for getFeedback cannot be used....
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [feedback, setFeedback] = useState<any | null>(null);
  useEffect(() => {
    setFeedback(Sentry.getFeedback());
  }, []);

  const buttonRef = useRef<HTMLButtonElement | null>(null);
  useEffect(() => {
    if (feedback) {
      const unsubscribe = feedback.attachTo(buttonRef.current);
      return unsubscribe;
    }
    return () => {};
  }, [feedback]);

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            type='button'
            ref={buttonRef}
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
