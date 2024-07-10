'use client';
import { Section } from '@/components/section';
import ServerStatus from './server-status';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';
import LocalServerSettings from './local-server-settings';


export default function StatusSection() {
  const isInDesktopApp = useIsInDesktopApp();

  return (
    <Section title="Status">
      <ServerStatus />
      { isInDesktopApp && <LocalServerSettings /> }
    </Section>
  );
}
