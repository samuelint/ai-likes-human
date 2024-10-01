import { ModelCard } from '@/components/model-card';
import { Section } from '@/components/section';
import { useLocalModelsList } from '@/lib/models/use-local-models-list';


export default function LocalModelsSection() {
  const data = useLocalModelsList();

  return (
    <Section title="Local Models">
      { data?.map((model) => (
        <ModelCard className='w-3/12' key={model.name} model={model.name} vendor={model.type} metadata={{ path: model.local_path }} />
      )) }
    </Section>
  );
}
