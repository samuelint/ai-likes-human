import { TooltipProvider } from '@radix-ui/react-tooltip';
import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip';
import { toFromNowFormattedDate } from '@/lib/date';
import { ReactNode } from 'react';


interface Props {
  run?: Pick<Run, 'model' | 'created_at'> | null
  children?: ReactNode
}

export function MessageRunDetailsTooltip({ run, children }: Props) {
  return (<TooltipProvider delayDuration={400}>
    <Tooltip>
      <TooltipTrigger asChild>
        { children }
      </TooltipTrigger>
      { run && <TooltipContent side='bottom'>
        <div className='flex flex-col font-light text-slate-600 text-xs select-none'>
          <span>{ run.model }</span>
          <span>{ toFromNowFormattedDate(run.created_at) }</span>
        </div>
      </TooltipContent>}
    </Tooltip>
  </TooltipProvider>);
}
