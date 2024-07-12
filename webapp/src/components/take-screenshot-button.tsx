import { Button } from './ui/button';
import { ScreenshotIcon } from './icon/screenshot';
import { SpinnerLoading } from '@/components/ui/loading';
import { cn } from '@/lib/utils';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';


interface Props {
  className?: string
  onClick?: () => void
  isLoading?: boolean
}

export function TakeScreenshotButton({ className, onClick, isLoading }: Props) {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant='ghost'
            type='button'
            onClick={onClick}
            className={cn('p-2', className)}
            disabled={isLoading}
          >
            {isLoading ? <SpinnerLoading className='w-6 h-6'/> : <ScreenshotIcon className='w-6 h-6'/>}
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          Take Screenshot
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>


  );
}
