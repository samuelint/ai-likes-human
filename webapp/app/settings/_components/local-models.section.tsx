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
        <div key={model.name}>
          <div>{model.name}</div>
          <div>{model.type}</div>
          <div>{model.local_files.q4_gguf_filepath}</div>
          <div>{model.local_files.fp16_gguf_filepath}</div>
        </div>
      )) }
    </Section>
  );
}
