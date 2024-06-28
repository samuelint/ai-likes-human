import TauriDebug from './components/tauri-debug';
import { Section } from '../../components/section';
import { H1 } from '../../components/h1';


export default function Settings() {
  return (
    <div>
      <H1>Settings</H1>
      <Section title="API Keys"></Section>
      <Section title="Internal Server"><TauriDebug /></Section>
    </div>
  );
}
