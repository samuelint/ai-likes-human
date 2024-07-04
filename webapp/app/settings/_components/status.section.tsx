'use client';
import { Section } from '@/components/section';
import ServerStatus from './server-status';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';
import LocalServerSettings from './local-server-settings';


export default function StatusSection() {
  const isInDesktopAppFn = useIsInDesktopApp();

  return (
    <Section title="Status">
      <ServerStatus />
      { isInDesktopAppFn() && <LocalServerSettings /> }
    </Section>
  );
}
