'use client';
import LocalServerSettings from './components/local-server-settings';
import { Section } from '../../components/section';
import { H1 } from '../../components/h1';
import ServerStatus from './components/server-status';
import { ConfigurationKvEditor } from './components/configuration-kv-editor';
import { useIsInDesktopApp } from '@/lib/is-in-desktop-app';


export default function Settings() {
  const isInDesktopAppFn = useIsInDesktopApp();

  return (
    <div>
      <H1>Settings</H1>
      <Section title="API Keys">
        <ConfigurationKvEditor kv_key="OPENAI_API_KEY"/>
        <ConfigurationKvEditor kv_key="ANTHROPIC_API_KEY"/>
        <ConfigurationKvEditor kv_key="SERPER_API_KEY"/>
      </Section>
      <Section title="Status">
        <ServerStatus />
        { isInDesktopAppFn() && <LocalServerSettings /> }
      </Section>
    </div>
  );
}
