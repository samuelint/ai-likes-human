import * as React from 'react';

import { Button } from '@/components/ui/button';
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from '@/components/ui/drawer';
import { ModelSettingsChoicesDto, ModelSettingsDto, ModelSettingsForm, OnModelSelectorSubmit } from './model-settings-form';


interface Props {
  choices: ModelSettingsChoicesDto
  defaultValues?: Partial<ModelSettingsDto>
  onSubmit?: OnModelSelectorSubmit;
  children?: React.ReactNode;
}

export function ModelSelectorModal({ onSubmit, children, ...props }: Props) {
  const [open, setOpen] = React.useState(false);

  const onSubmitProxy = React.useCallback<OnModelSelectorSubmit>((data) => {
    setOpen(false);
    onSubmit?.(data);
  }, [onSubmit]);

  return (
    <Drawer open={open} onOpenChange={setOpen}>
      <DrawerTrigger asChild>
        { children }
      </DrawerTrigger>
      <DrawerContent>
        <div className="mx-auto w-full max-w-sm">
          <DrawerHeader className="text-left">
            <DrawerTitle>Model Settings</DrawerTitle>
          </DrawerHeader>
          <div className="p-4 pb-0">
            <ModelSettingsForm {...props} onSubmit={onSubmitProxy} />
          </div>
          <DrawerFooter className="pt-2">
            <DrawerClose asChild>
              <Button variant="outline">Cancel</Button>
            </DrawerClose>
          </DrawerFooter>
        </div>
      </DrawerContent>
    </Drawer>
  );
}
