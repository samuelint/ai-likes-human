import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';


export default function AdvancedSection() {
  return (
    <Section title="Advanced">
      <ConfigurationKvEditor label='LLM Model' kv_key="DEFAULT_LLM_MODEL"/>
      <ConfigurationKvEditor label='Default LLM Temperature' kv_key="DEFAULT_LLM_TEMPERATURE"/>
    </Section>
  );
}
