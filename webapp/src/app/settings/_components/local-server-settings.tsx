import StopLocalServer from './stop-local-server';
import StartLocalServer from './start-local-server';
import { useServerStatus } from '@/lib/local-server-context';


export default function LocalServerSettings() {
  const { isUp } = useServerStatus();

  return (<>
    { isUp ? <StopLocalServer /> : <StartLocalServer /> }
  </>);
}
