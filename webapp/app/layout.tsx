import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

import appConfig from '@/app.config';
import { cn } from '@/lib/utils';
import { MainLayout } from './main-layout';


const inter = Inter({ subsets: ['latin'] });

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
