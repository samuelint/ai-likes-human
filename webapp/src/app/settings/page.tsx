import { useIsInDesktopApp, useScrollToSectionUsingRouteHash } from '@/lib/utils';
import ApiKeysSection from './_components/api-keys.section';
import InternalInfo from './_components/internal-info.section';

export function Settings() {
  const isInDesktopApp = useIsInDesktopApp();

  useScrollToSectionUsingRouteHash();

  return (
    <div className='w-full flex flex-col gap-4'>
      <ApiKeysSection />
      { process.env.NODE_ENV === 'development' && isInDesktopApp && <InternalInfo /> }
    </div>
  );
}
