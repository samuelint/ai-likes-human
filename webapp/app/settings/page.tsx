'use client';
import LocalServerSettings from './components/local-server-settings';
import { Section } from '../../components/section';
import { H1 } from '../../components/h1';
import ServerStatus from './components/server-status';


export default function Settings() {
  return (
    <div>
      <H1>Settings</H1>
      <Section title="API Keys"></Section>
      <Section title="ServerStatus">
        <ServerStatus />
        { window.__TAURI_IPC__ != null && <LocalServerSettings /> }
      </Section>
    </div>
  );
}
