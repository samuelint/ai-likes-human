import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

import { Toaster } from '@/components/ui/toaster';
import Header from './_components/header';
import appConfig from '@/app.config';
import { cn } from '@/lib/utils';


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
        <div className='fixed w-full'>
          <Header />
        </div>
        <div className='w-full h-full pt-10'>
          {children}
        </div>
        <Toaster />
      </body>
    </html>
  );
}
