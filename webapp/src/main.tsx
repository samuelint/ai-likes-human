import './globals.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import Home from '@/app/page';
import { MainLayout } from './app/main-layout';
import { Route, Switch } from 'wouter';
import Settings from '@/app/settings/page';
import { Thread } from '@/app/thread/thread';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <MainLayout>
      <Switch>
        <Route path="/"><Home /></Route>
        <Route path="/thread/:threadId">
          {(params) => <Thread threadId={params.threadId} />}
        </Route>
        <Route path="/settings"><Settings /></Route>
      </Switch>
    </MainLayout>
  </React.StrictMode>,
);
