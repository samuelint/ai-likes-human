import { Section } from '@/components/section';
import InferenceServerUrl from './inference-server-url';
import InferenceServerStatus from './inference-server-status';
import AppDataDirectory from './app-data-directory';


export default function InferenceServerSection() {

  return (
    <Section id='internal-info' title="Internal Info">
      <InferenceServerUrl />
      <InferenceServerStatus />
      <AppDataDirectory />
    </Section>
  );
}
