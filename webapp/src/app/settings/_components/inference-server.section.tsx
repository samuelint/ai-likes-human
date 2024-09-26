import { Section } from '@/components/section';
import InferenceServerUrl from './inference-server-url';
import InferenceServerStatus from './inference-server-status';



export default function InferenceServerSection() {

  return (
    <Section id='inference-server' title="Inference Server">
      <InferenceServerUrl />
      <InferenceServerStatus />
    </Section>
  );
}
