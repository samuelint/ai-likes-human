import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';


export default function ApiKeysSection() {
  return (
    <Section id='api-keys' title="API Keys">
      <ConfigurationKvEditor kv_key="OPENAI_API_KEY" isSecret />
      <ConfigurationKvEditor kv_key="ANTHROPIC_API_KEY" isSecret />
      <ConfigurationKvEditor kv_key="SERPER_API_KEY" isSecret />
    </Section>
  );
}
