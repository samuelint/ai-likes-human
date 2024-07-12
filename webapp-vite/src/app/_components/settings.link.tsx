import { Link } from "wouter";
import { SettingsIcon } from 'lucide-react';


export default function SettingLink() {
  return (
    <Link href="/settings">
      <SettingsIcon className='w-4 h-4'/>
    </Link>
  );
}
