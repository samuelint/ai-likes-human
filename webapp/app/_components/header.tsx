import BrandLink from './brand.link';
import SettingLink from './settings.link';
import PrivacyToggle from './privacy-toggle';


export default function Header() {
  return (
    <header className="w-full bg-gray-900 text-white py-2 px-6 flex items-center">
      <div className="w-full flex items-center space-x-4 justify-between">
        <div className='flex items-center gap-4'>
          <BrandLink />
        </div>
        <div className='flex items-center gap-4'>
          <PrivacyToggle />
          <SettingLink />
        </div>
      </div>
    </header>
  );
}
