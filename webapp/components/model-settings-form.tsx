'use client';

import { zodResolver } from '@hookform/resolvers/zod';
import { SubmitHandler, useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';


export interface ModelSettingsDto {
  model: string
}

export interface ModelSettingsChoicesDto {
  models: readonly [string, ...string[]]
}

export type OnModelSelectorSubmit = SubmitHandler<ModelSettingsDto>;
interface Props {
  choices: ModelSettingsChoicesDto
  defaultValues?: Partial<ModelSettingsDto>
  onSubmit?: OnModelSelectorSubmit
}

export function ModelSettingsForm({ choices, defaultValues, onSubmit }: Props) {
  const FormSchema = z.object({
    model: z.enum(choices.models, {
      required_error: 'You need to select a model.',
    }),
  });

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues,
  });

  return (
    <Form {...form}>
      <form onSubmit={onSubmit && form.handleSubmit(onSubmit)} className="space-y-6">
        <FormField
          control={form.control}
          name="model"
          render={({ field }) => (
            <FormItem className="space-y-3">
              <FormLabel>Model</FormLabel>
              <FormControl>
                <RadioGroup
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  className="flex flex-col space-y-1"
                >
                  { choices.models.map((model) => (
                    <FormItem key={model} className="flex items-center space-x-3 space-y-0">
                      <FormControl>
                        <RadioGroupItem value={model} />
                      </FormControl>
                      <FormLabel className="font-normal">
                        { model }
                      </FormLabel>
                    </FormItem>))}
                </RadioGroup>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button className='w-full' type="submit">Save</Button>
      </form>
    </Form>
  );
}
