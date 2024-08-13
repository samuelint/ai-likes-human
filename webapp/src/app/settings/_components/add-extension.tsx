import { useErrorNotification } from '@/app/_components/use-error-notification';
import { AddLocalExtensionForm } from '@/components/add-local-extension-form';
import { Button } from '@/components/ui/button';

import { useExtensions } from '@/lib/extension/use-extensions';
import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';

export function AddExtension() {
  const { upload, error } = useExtensions();

  useErrorNotification(error);

  return (
    <AddLocalExtensionForm onSubmit={upload} />
  );
}

export function AddExtensionDialog() {
  return (
    <Dialog>
      <DialogTrigger>
        <Button className='w-full'>Add Extension</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <AddExtension />
      </DialogContent>
    </Dialog>
  );
}