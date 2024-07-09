import { TooltipProvider } from '@radix-ui/react-tooltip';
import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';
import { Button } from './ui/button';
import { Info } from 'lucide-react';
import { toFromNowFormattedDate } from '@/lib/date';


interface Props {
  run: Pick<Run, 'model' | 'created_at'>
}

export function MessageRunDetails({ run }: Props) {
  return (
    <TooltipProvider delayDuration={100}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="ghost" size="icon" className='w-6 h-6 text-slate-200 hover:text-slate-500'><Info className='w-4 h-4'/></Button>
        </TooltipTrigger>
        <TooltipContent side='bottom'>
          <div className='flex flex-col font-light text-slate-600 text-xs select-none'>
            <span>{ run.model }</span>
            <span>{ toFromNowFormattedDate(run.created_at) }</span>
          </div>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>

  );
}
