import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';


export default function GeneralSection() {
  return (
    <Section title="General">
      <ConfigurationKvEditor label='Name' kv_key="USERNAME"/>
    </Section>
  );
}
