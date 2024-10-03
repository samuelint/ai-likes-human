import { ReadonlyKV } from '@/components/readonly-kv';
import { getSelectedProfiles } from '@/lib/core-api/tauri';
import { useAsync } from 'react-use';



export default function InferenceSelectedProfiles() {
  const { value } = useAsync(() => getSelectedProfiles());

  const profiles = value?.map((profile) => profile.name).join(', ');
  return (
    <ReadonlyKV name="Profiles">{profiles}</ReadonlyKV>
  );
}
