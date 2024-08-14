import { useErrorNotification } from '@/app/_components/use-error-notification';
import { AddLocalExtensionForm, OnExtensionSubmit } from '@/components/add-local-extension-form';
import { Button } from '@/components/ui/button';

import { useExtensions } from '@/lib/extension/use-extensions';
import { Dialog, DialogContent, DialogTrigger } from '@/components/ui/dialog';
import { useState } from 'react';

interface AddExtensionProps {
  onSubmit?: () => void;
}

export function AddExtension({ onSubmit }: AddExtensionProps) {
  const { upload, error } = useExtensions();

  useErrorNotification(error);

  const onInternalSubmit: OnExtensionSubmit = (file) => {
    upload(file);
    onSubmit && onSubmit();
  };

  return (
    <AddLocalExtensionForm onSubmit={onInternalSubmit} />
  );
}

export function AddExtensionDialog() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger>
        <Button className='w-full'>Add Extension</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <AddExtension onSubmit={() => setIsOpen(false)} />
      </DialogContent>
    </Dialog>
  );
}