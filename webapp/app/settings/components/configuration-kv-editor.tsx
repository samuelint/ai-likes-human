'use client';

import { Button } from '@/components/ui/button';
import { ErrorDetails } from '@/components/ui/error';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useToast } from '@/components/ui/use-toast';
import { useConfigurationKV } from '@/lib/use-configuration-kv';


interface Props {
  label?: string
  kv_key: string
}

export function ConfigurationKvEditor({ label, kv_key }: Props) {
  const { toast } = useToast();
  const { data, error, isLoading, mutate } = useConfigurationKV(kv_key);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    event.preventDefault();

    const form = event.target;
    // @ts-expect-error - form is compatible
    const formData = new FormData(form);
    const entries = Object.fromEntries(formData.entries());
    const newValue = entries[kv_key] as string;

    mutate({ key: kv_key, value: newValue })
      .then(() => toast({ title: `${label ?? kv_key} Saved` }))
      .catch((e) => toast({
        title: `${label ?? kv_key} Saved`,
        description: e.message,
        variant: 'destructive',
      }));
  };


  return (
    <>
      <form
        method="post"
        onSubmit={handleSubmit}
        className="grid w-full max-w-sm items-center gap-1.5"
      >
        <fieldset disabled={isLoading || error}>
          <Label htmlFor={kv_key}>{label || kv_key}</Label>
          <ErrorDetails error={error} />
          <div className="flex w-full max-w-sm items-center space-x-2">
            <Input name={kv_key} id={kv_key} placeholder={kv_key} defaultValue={data?.value} />
            <Button type="submit">Save</Button>
          </div>
        </fieldset>
      </form>
    </>
  );
}
