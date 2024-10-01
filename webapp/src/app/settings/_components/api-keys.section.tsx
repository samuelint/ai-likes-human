import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';
import { LLM_API_KEYS_KEYS } from '@/app.config';


export default function ApiKeysSection() {
  return (
    <Section id='api-keys' title="API Keys">
      { LLM_API_KEYS_KEYS.map((key) => <ConfigurationKvEditor key={key} kv_key={key} isSecret />) }
    </Section>
  );
}
