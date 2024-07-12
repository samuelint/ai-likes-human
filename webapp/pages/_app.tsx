import { MainLayout } from '@/app/main-layout';
import { AppProps } from 'next/app';


export default function MyApp({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <MainLayout>
      <Component {...pageProps} />
    </MainLayout>
  );
}