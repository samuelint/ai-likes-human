import { Link } from 'wouter';
import { SettingsIcon } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';


export default function SettingLink() {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <Link href="/settings">
            <SettingsIcon className='w-4 h-4'/>
          </Link>
        </TooltipTrigger>
        <TooltipContent>
          Settings
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
