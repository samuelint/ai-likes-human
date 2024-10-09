import { Section } from '@/components/section';
import { ConfigurationKvEditor } from './configuration-kv-editor';
import { LLM_VENDORS_CONFIGURATIONS } from '@/app.config';
import ApiKeysHint from './api-key-hint';


export default function ApiKeysSection() {
  return (
    <Section id='api-keys' title="API Keys">
      { LLM_VENDORS_CONFIGURATIONS
        .filter((vendor) => vendor.api_key_key)
        .map((vendor) => <ConfigurationKvEditor
          key={vendor.name}
          kv_key={vendor.api_key_key!}
          isSecret
          hint={<ApiKeysHint how_to_get_api_key_url={vendor.how_to_get_api_key_url} />}
        />) }
    </Section>
  );
}
