'use client';

import { Button } from '@/components/ui/button';
import { FormatError } from '@/components/ui/error';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useConfigurationKV } from '@/lib/use-configuration-kv';


interface Props {
  kv_key: string
}

export function ConfigurationKvEditor({ kv_key }: Props) {
  const { data, error, isLoading, mutate } = useConfigurationKV(kv_key);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    event.preventDefault();

    const form = event.target;
    // @ts-expect-error - form is compatible
    const formData = new FormData(form);
    const entries = Object.fromEntries(formData.entries());
    const newValue = entries[kv_key] as string;

    mutate({ key: kv_key, value: newValue });
  };

  return (
    <>
      <form
        method="post"
        onSubmit={handleSubmit}
        className="grid w-full max-w-sm items-center gap-1.5"
      >
        <fieldset disabled={isLoading || error}>
          <Label htmlFor={kv_key}>{kv_key}</Label>
          <div className="flex w-full max-w-sm items-center space-x-2">
            <Input name={kv_key} id={kv_key} placeholder={kv_key} defaultValue={data?.value} />
            <Button type="submit">Save</Button>
          </div>
        </fieldset>
      </form>
      <FormatError error={error} />
    </>
  );
}
