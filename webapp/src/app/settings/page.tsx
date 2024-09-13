import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';
import ApiKeysSection from './_components/api-keys.section';
import GeneralSection from './_components/general.section';
import LocalModelsSection from './_components/local-models.section';
import AdvancedSection from './_components/advanced.section';
import { PageSection } from '@/components/page-section';
import { useScrollToSectionUsingRouteHash } from '@/lib/use-scroll-to-id';
import ExtensionsSection from './_components/extensions-section';


export default function Settings() {
  const isInDesktopApp = useIsInDesktopApp();
  const showLocalModelsSection = isInDesktopApp || process.env.NODE_ENV === 'development';

  useScrollToSectionUsingRouteHash();

  return (
    <PageSection title="Settings">
      <div className='flex flex-col gap-6'>
        <GeneralSection />
        <ApiKeysSection />
        <ExtensionsSection />
        <AdvancedSection />
        { showLocalModelsSection && <LocalModelsSection />}
      </div>
    </PageSection>
  );
}
