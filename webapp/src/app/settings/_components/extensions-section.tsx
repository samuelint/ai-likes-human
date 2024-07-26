import { useErrorNotification } from '@/app/_components/use-error-notification';
import { AddLocalExtensionForm } from '@/components/add-local-extension-form';
import { ExtensionCard } from '@/components/extension-card';
import { Section } from '@/components/section';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { useExtensions } from '@/lib/extension/use-extensions';

export default function ExtensionsSection() {
  return (
    <Section id='extensions' title="Extensions">
      <AvailableExtensions />
      <AddExtensions />
    </Section>
  );
}


export function AvailableExtensions() {
  const { data, isLoading, error, remove } = useExtensions();

  return (
    <>
      <div className='grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4'>
        { data?.map((extension) => (
          <ExtensionCard key={extension.name} extension={extension}>
            <Button variant="destructive" onClick={() => remove(extension)}>Delete</Button>
          </ExtensionCard>
        ))}
      </div>

      { error && <ErrorDetails error={error} /> }
      { isLoading && <ThreeDotsLoading /> }
    </>
  );
}

export function AddExtensions() {
  const { upload, error } = useExtensions();

  useErrorNotification(error);

  return (
    <Card className='p-6 flex flex-col gap-4'>
      <AddLocalExtensionForm onSubmit={upload} />
    </Card>
  );
}
