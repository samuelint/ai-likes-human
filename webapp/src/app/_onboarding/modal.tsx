import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog';
import OnBoarding from './onboarding';
import { Button } from '@/components/ui/button';
import { useMustOnboard } from './use-must-onboard';
import { useEffect, useState } from 'react';

export function OnBoardingModal() {
  const { value: mustOnboard } = useMustOnboard();
  const [openOnboarding, setOpenOnboarding] = useState(false);

  useEffect(() => {
    if (mustOnboard) {
      setOpenOnboarding(mustOnboard);
    }
  }, [mustOnboard]);

  const onOpenChange = (open: boolean) => {
    setOpenOnboarding(open);
  };

  return (
    <Dialog open={openOnboarding}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Onboarding</DialogTitle>
          <DialogDescription>
            At least one API Key of one of the providers must be set.
          </DialogDescription>
        </DialogHeader>
        <div className='max-w-full'>
          <OnBoarding />
        </div>
        <DialogFooter>
          <Button type="submit" disabled={mustOnboard} onClick={() => onOpenChange(false)}>Done</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
