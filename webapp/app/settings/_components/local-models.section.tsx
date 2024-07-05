import { ModelCard } from '@/components/model-card';
import { Section } from '@/components/section';
import { ErrorDetails } from '@/components/ui/error';
import { ThreeDotsLoading } from '@/components/ui/loading';
import { useLocalModelsList } from '@/lib/use-local-models-list';


export default function LocalModelsSection() {
  const { data, isLoading, error } = useLocalModelsList();

  return (
    <Section title="Local Models">
      { error && <ErrorDetails error={error} /> }
      { isLoading && <ThreeDotsLoading /> }
      { data?.map((model) => (
        <ModelCard className='w-3/12' key={model.name} model={model.name} vendor={model.type} metadata={{ path: model.local_path }} />
      )) }
    </Section>
  );
}
