import Home from '@/app/page';
import { Route, Switch } from 'wouter';
import { Thread } from '@/app/thread/thread';

export default function Routes() {
  return (
    <Switch>
      <Route path="/"><Home /></Route>
      <Route path="/thread/:threadId">
        {(params) => <Thread threadId={params.threadId} />}
      </Route>
    </Switch>
  );
}
