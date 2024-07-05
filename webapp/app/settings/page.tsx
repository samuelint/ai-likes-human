'use client';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';
import { H1 } from '../../components/h1';
import ApiKeysSection from './_components/api-keys.section';
import GeneralSection from './_components/general.section';
import LocalModelsSection from './_components/local-models.section';
import StatusSection from './_components/status.section';


export default function Settings() {
  const isInDesktopApp = useIsInDesktopApp();
  const showLocalModelsSection = isInDesktopApp || process.env.NODE_ENV === 'development';

  return (
    <div>
      <H1>Settings</H1>
      <GeneralSection />
      <ApiKeysSection />
      { showLocalModelsSection && <LocalModelsSection />}
      <StatusSection />
    </div>
  );
}
