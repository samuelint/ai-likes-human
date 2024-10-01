import { useIsInDesktopApp, useScrollToSectionUsingRouteHash } from '@/lib/utils';
import ApiKeysSection from './_components/api-keys.section';
import { PageSection } from '@/components/page-section';
import InternalInfo from './_components/internal-info.section';


export default function Settings() {
  const isInDesktopApp = useIsInDesktopApp();

  useScrollToSectionUsingRouteHash();

  return (
    <PageSection title="Settings">
      <div className='flex flex-col gap-6'>
        <ApiKeysSection />
        { process.env.NODE_ENV === 'development' && isInDesktopApp && <InternalInfo /> }
      </div>
    </PageSection>
  );
}
