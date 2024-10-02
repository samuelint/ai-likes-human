import { ReadonlyKV } from '@/components/readonly-kv';
import { getAppDirectoryPath } from '@/lib/core-api/tauri';

import { useAsync } from 'react-use';


export default function AppDataDirectory() {
  const { value } = useAsync(() => getAppDirectoryPath());

  return (
    <ReadonlyKV name='App Data Path'>{value}</ReadonlyKV>
  );
}
