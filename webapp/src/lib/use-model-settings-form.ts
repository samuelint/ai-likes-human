import { ModelSettingsChoicesDto, ModelSettingsDto, OnModelSelectorSubmit } from '@/components/model-settings-form';
import { useCallback } from 'react';
import { useLLMModel } from './use-llm-model';
import { useAvailableModels } from './use-available-models';


interface ModelSettingsFormControls {
  choices: ModelSettingsChoicesDto
  defaultValues?: Partial<ModelSettingsDto>
  handleSubmit?: OnModelSelectorSubmit
}

export function useModelSettingsFormControls(): ModelSettingsFormControls {
  const { data: model, mutate } = useLLMModel();
  const { data: availableModels } = useAvailableModels();

  const handleSubmit = useCallback<OnModelSelectorSubmit>(async (newData) => {
    await mutate(newData.model);
  }, [mutate]);

  return {
    defaultValues: {
      model: model,
    },
    choices: {
      models: availableModels,
    },
    handleSubmit,
  };
}