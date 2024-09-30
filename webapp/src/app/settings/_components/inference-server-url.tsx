import { ReadonlyKV } from '@/components/readonly-kv';
import { getInferenceServerUrl } from '@/lib/tauri-command';
import { useAsync } from 'react-use';



export default function InferenceServerUrl() {
  const { value } = useAsync(() => getInferenceServerUrl());

  return (
    <ReadonlyKV name='Url'>{value}</ReadonlyKV>
  );
}
