'use client';

import { LockIcon } from '@/components/icon/lock.icon';
import { UnlockIcon } from '@/components/icon/unlock.icon';
import { Button } from '@/components/ui/button';
import { useState } from 'react';


export default function PrivacyToggle() {
  const [isPrivacyEnabled, setIsPrivacyEnabled] = useState(false);

  return (
    <Button className='h-fit p-0' onClick={() => setIsPrivacyEnabled((prev) => !prev)}>
      { isPrivacyEnabled ? <LockIcon className='w-4 h-4' /> : <UnlockIcon className='w-4 h-4'/>}
    </Button>
  );
}
