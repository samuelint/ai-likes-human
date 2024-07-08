'use client';
import { Section } from '@/components/section';
import ServerStatus from './server-status';
import { useIsInDesktopAppFn } from '@/lib/is-in-desktop-app';
import LocalServerSettings from './local-server-settings';


export default function StatusSection() {
  const isInDesktopAppFn = useIsInDesktopAppFn();

  return (
    <Section title="Status">
      <ServerStatus />
      { isInDesktopAppFn() && <LocalServerSettings /> }
    </Section>
  );
}
