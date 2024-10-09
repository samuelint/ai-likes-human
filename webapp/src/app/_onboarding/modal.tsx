import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog';
import OnBoarding from './onboarding';
import { Button } from '@/components/ui/button';
import { useMustOnboard } from './use-must-onboard';
import { useEffect, useState } from 'react';
import { FullScreenDialogContent } from '@/components/ui/full-screen-dialog';

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
      <FullScreenDialogContent className="[&>button]:hidden">
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
      </FullScreenDialogContent>
    </Dialog>
  );
}
