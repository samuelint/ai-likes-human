import { ReadonlyKV } from '@/components/readonly-kv';
import { Section } from '@/components/section';
import { getInferenceServerUrl } from '@/lib/tauri-command';
import { useAsync } from 'react-use';



export default function InferenceServerSection() {
  const { value } = useAsync(() => getInferenceServerUrl());

  return (
    <Section id='inference-server' title="Inference Server">
      <ReadonlyKV name='Internal Inference Url'>{value}</ReadonlyKV>
    </Section>
  );
}
