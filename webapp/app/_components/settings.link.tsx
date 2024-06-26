import { SettingsIcon } from 'lucide-react';
import Link from 'next/link';


export default function SettingLink() {
  return (
    <Link href="/settings">
      <SettingsIcon className='w-4 h-4'/>
    </Link>
  );
}
