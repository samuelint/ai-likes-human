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
import { UploadExtensionDto, UploadExtensionSchema } from '@/lib/extension/extension.dto';


export type OnExtensionSubmit = SubmitHandler<UploadExtensionDto>;

interface Props {
  onSubmit?: OnExtensionSubmit
}

export function AddLocalExtensionForm({ onSubmit }: Props) {

  const form = useForm<UploadExtensionDto>({
    resolver: zodResolver(UploadExtensionSchema),
  });

  return (
    <Form {...form} >
      <form onSubmit={onSubmit && form.handleSubmit(onSubmit)} className="space-y-6">
        <FormField
          control={form.control}
          name="file"
          render={({ field: { value, onChange, ...fieldProps } }) => (
            <FormItem className="space-y-3">
              <FormLabel>Upload Extension</FormLabel>
              <FormControl>
                <Input
                  {...fieldProps}
                  type="file"
                  accept=".whl"
                  placeholder='Select a .whl file'
                  onChange={(event) =>
                    onChange(event.target.files && event.target.files[0])
                  }
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )} />
        <Button type="submit">Add</Button>
      </form>
    </Form>
  );
}
