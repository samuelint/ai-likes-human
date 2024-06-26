import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

import { twMerge } from 'tailwind-merge';
import { Toaster } from '@/components/ui/toaster';
import Header from './_components/header';
import appConfig from '@/app.config';


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
      <body className={twMerge(inter.className, 'flex flex-col h-screen')}>
        <Header />
        {children}
        <Toaster />
      </body>
    </html>
  );
}
