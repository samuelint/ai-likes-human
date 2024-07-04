import { inter } from '@/app/main-layout';
import { cn } from '@/lib/utils';
import { Html, Head, Main, NextScript } from 'next/document';


export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className={cn(inter.className, 'h-screen w-screen')}>
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}