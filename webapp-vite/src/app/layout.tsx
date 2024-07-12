import type { Metadata } from 'next';

import appConfig from '@/app.config';
import { cn } from '@/lib/utils';
import { MainLayout, inter } from './main-layout';


export const metadata: Metadata = {
  title: appConfig.app_title,
  description: appConfig.app_description,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={cn(inter.className, 'h-screen w-screen')}>
        <MainLayout>
          {children}
        </MainLayout>
      </body>
    </html>
  );
}
