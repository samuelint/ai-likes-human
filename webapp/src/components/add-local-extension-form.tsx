import { Button } from './ui/button';
import { Input } from '@/components/ui/input';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { useForm, SubmitHandler } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { ACCEPTED_EXTENSIONS_FILE_EXTENSION, UploadExtensionDto, UploadExtensionSchema } from '@/lib/extension/extension.dto';
import { useRef } from 'react';


export type OnExtensionSubmit = SubmitHandler<UploadExtensionDto>;

interface Props {
  onSubmit?: OnExtensionSubmit
  acceptedExtensions?: readonly string[]
}

export function AddLocalExtensionForm({ onSubmit, acceptedExtensions = ACCEPTED_EXTENSIONS_FILE_EXTENSION }: Props) {

  const form = useForm<UploadExtensionDto>({
    resolver: zodResolver(UploadExtensionSchema),
  });

  const inputFileRef = useRef<HTMLInputElement | null>(null);

  const reset = () => {
    form.reset();

    // Workaround since the input file name is not reset
    if (inputFileRef.current) inputFileRef.current.value = '';
  };

  const handleSubmit: OnExtensionSubmit = async (data, event) => {
    if (onSubmit) {
      await onSubmit(data, event);
    }
    reset();
  };

  return (
    <Form {...form}>
      <h3 className="mb-4 text-lg font-medium">Add Extension</h3>
      <form onSubmit={onSubmit && form.handleSubmit(handleSubmit)} className="space-y-6">
        <FormField
          control={form.control}
          name="file"
          render={({ field: { value, onChange, ...fieldProps } }) => (
            <FormItem className="space-y-3">
              <FormLabel>Extension File ({acceptedExtensions.join(', ')})</FormLabel>
              <FormControl>
                <Input
                  {...fieldProps}
                  ref={inputFileRef}
                  type="file"
                  className="cursor-pointer"
                  accept={acceptedExtensions.join(',')}
                  onChange={(event) =>
                    onChange(event.target.files && event.target.files[0])
                  }
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )} />
        <div className='flex flex-row gap-4'>
          <Button type="submit">Add</Button>
        </div>
      </form>
    </Form>
  );
}
