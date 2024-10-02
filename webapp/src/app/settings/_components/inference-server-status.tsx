import { ReadonlyKV } from '@/components/readonly-kv';
import { isInferenceServerRunning } from '@/lib/core-api/tauri';
import { useAsync } from 'react-use';



export default function InferenceServerStatus() {
  const { value } = useAsync(() => isInferenceServerRunning());

  return (
    <ReadonlyKV name='Status'>{value ? 'Running' : 'Stopped'}</ReadonlyKV>
  );
}
