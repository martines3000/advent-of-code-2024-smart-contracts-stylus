import type { Metadata } from 'next';
import localFont from 'next/font/local';
import './globals.css';
import { Navbar } from '@/components/Navbar';
import { Providers } from '@/components/Providers';
import { cn } from '@/lib/utils';

const geistSans = localFont({
  src: './fonts/GeistVF.woff',
  variable: '--font-geist-sans',
  weight: '100 900',
});
const geistMono = localFont({
  src: './fonts/GeistMonoVF.woff',
  variable: '--font-geist-mono',
  weight: '100 900',
});

export const metadata: Metadata = {
  title: 'Advent of Code 2024 - Smart Contracts',
  description:
    'Solutions to the Advent of Code 2024 challenges using smart contracts',
  robots: {
    index: true,
    follow: true,
  },
  openGraph: {
    description:
      'Solutions to the Advent of Code 2024 challenges using smart contracts',
    siteName: 'Advent of Code 2024 - Smart Contracts',
    title: 'Advent of Code 2024 - Smart Contracts',
    type: 'article',
    url: '/',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Advent of Code 2024 - Smart Contracts',
    description:
      'Solutions to the Advent of Code 2024 challenges using smart contracts',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={cn(geistSans.variable, geistMono.variable, 'antialiased ')}
      >
        <Providers>
          <div className="min-h-screen h-screen font-[family-name:var(--font-geist-sans)]">
            <main className="h-full w-full flex flex-col">
              <Navbar />
              <div className="px-8 pb-8 h-full w-full">{children}</div>
            </main>
          </div>
        </Providers>
      </body>
    </html>
  );
}
