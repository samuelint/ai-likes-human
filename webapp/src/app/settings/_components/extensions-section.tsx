import { ExtensionTable } from '@/components/extension-table';
import { Section } from '@/components/section';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { useExtensions } from '@/lib/extension/use-extensions';
import { AddExtensionDialog } from './add-extension';
import { PropsWithChildren } from 'react';

export default function ExtensionsSection() {
  return (
    <Section id='extensions' title="Extensions">
      <Card>
        <AvailableExtensions>
          <AddExtensionDialog />
        </AvailableExtensions>
      </Card>
    </Section>
  );
}


export function AvailableExtensions({ children }: PropsWithChildren) {
  const { data, isLoading, error, remove } = useExtensions();

  return (
    <ExtensionTable
      extensions={data}
      Actions={(extension) => <>
        <Button onClick={() => remove(extension)}>Load / Unload</Button>
        <Button variant="destructive" onClick={() => remove(extension)}>Delete</Button>
      </>}
    >
      { error && <ErrorDetails error={error} /> }
      { isLoading && <ThreeDotsLoading /> }
      { children }
    </ExtensionTable>
  );
}


