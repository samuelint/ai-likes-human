import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';


export default function AdvancedSection() {
  return (
    <Section id='advanced' title="Advanced">
      <ConfigurationKvEditor label='Default LLM Temperature' kv_key="DEFAULT_LLM_TEMPERATURE"/>
    </Section>
  );
}
