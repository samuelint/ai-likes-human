import './globals.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { MainLayout } from './app/main-layout';
import Splashscreen from './components/splashscreen';
import Providers from './providers';
import Routes from './routes';
import { useServerStatus } from './lib/local-server-context';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Providers>
      <MainLayout>
        <App />
      </MainLayout>
    </Providers>
  </React.StrictMode>,
);

export default function App() {
  const { hasAlreadyBeenUp } = useServerStatus();

  return hasAlreadyBeenUp ? <Routes /> : <Splashscreen />;
}
