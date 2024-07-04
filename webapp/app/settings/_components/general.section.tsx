import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';


export default function GeneralSection() {
  return (
    <Section title="General">
      <ConfigurationKvEditor label='LLM Model' kv_key="DEFAULT_LLM_MODEL"/>
      <ConfigurationKvEditor label='Name' kv_key="USERNAME"/>
    </Section>
  );
}
