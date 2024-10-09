'use client';

import { SettingsIcon } from 'lucide-react';
import { Dialog, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { useState } from 'react';
import { Settings } from './page';
import { FullScreenDialogContent } from '@/components/ui/full-screen-dialog';


export function SettingsModal() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger>
        <SettingsIcon className='w-4 h-4'/>
      </DialogTrigger>
      <FullScreenDialogContent className="max-w-[calc(100vw-2rem)] max-h-[calc(100hw-2rem)]">
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
        </DialogHeader>
        <div className='max-w-full'>
          <Settings />
        </div>
      </FullScreenDialogContent>
    </Dialog>
  );
}
