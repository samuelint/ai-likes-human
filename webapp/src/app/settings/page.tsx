import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';
import ApiKeysSection from './_components/api-keys.section';
import GeneralSection from './_components/general.section';
import LocalModelsSection from './_components/local-models.section';
import AdvancedSection from './_components/advanced.section';
import { PageSection } from '@/components/page-section';
import { useScrollToSectionUsingRouteHash } from '@/lib/use-scroll-to-id';
import ExtensionsSection from './_components/extensions-section';
import InferenceServerSection from './_components/inference-server';


export default function Settings() {
  const isInDesktopApp = useIsInDesktopApp();

  useScrollToSectionUsingRouteHash();

  return (
    <PageSection title="Settings">
      <div className='flex flex-col gap-6'>
        <GeneralSection />
        <ApiKeysSection />
        <ExtensionsSection />
        { isInDesktopApp && <LocalModelsSection />}
        <AdvancedSection />
        { isInDesktopApp && <InferenceServerSection /> }
      </div>
    </PageSection>
  );
}
