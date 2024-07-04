import { H1 } from '../../components/h1';
import ApiKeysSection from './_components/api-keys.section';
import GeneralSection from './_components/general.section';
import StatusSection from './_components/status.section';


export default function Settings() {
  return (
    <div>
      <H1>Settings</H1>
      <GeneralSection />
      <ApiKeysSection />
      <StatusSection />
    </div>
  );
}
