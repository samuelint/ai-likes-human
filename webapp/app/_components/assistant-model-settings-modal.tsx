'use client';

import { ModelSelectorModal } from '@/components/model-settings-modal';
import { Button } from '@/components/ui/button';
import { useModelSettingsFormControls } from '@/lib/use-model-settings-form';


export default function AssistantModelSettingsModal() {
  const { choices, defaultValues, handleSubmit } = useModelSettingsFormControls();

  return (
    <ModelSelectorModal choices={choices} defaultValues={defaultValues} onSubmit={handleSubmit}>
      <Button variant='ghost' className='text-slate-400 text-xs py-0 px-2 h-fit'>{defaultValues?.model}</Button>
    </ModelSelectorModal>
  );
}
