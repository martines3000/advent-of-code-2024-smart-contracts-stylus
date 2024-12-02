import type { Metadata } from 'next';
import localFont from 'next/font/local';
import './globals.css';
import { Navbar } from '@/components/Navbar';
import { Providers } from '@/components/Providers';
import { cn } from '@/lib/utils';
import Image from 'next/image';

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
              <div className="px-8 py-8 h-full w-full">{children}</div>
              <div className="h-12 flex gap-4 justify-center items-center border-t-2 rounded-full border-primary">
                <a
                  href="https://github.com/martines3000/advent-of-code-2024-smart-contracts-stylus"
                  className="hover:cursor-pointer hover:text-primary hover:underline"
                >
                  Code
                </a>
                <a
                  href="https://twitter.com/MartinesXD"
                  className="hover:cursor-pointer hover:text-primary"
                >
                  <Image
                    src="/x-white.png"
                    height={12}
                    width={12}
                    alt="X logo"
                  />
                </a>
                <a
                  href="https://twitter.com/lutralabs_"
                  className="hover:cursor-pointer hover:text-primary hover:underline"
                >
                  Lutra Labs
                </a>
              </div>
            </main>
          </div>
        </Providers>
      </body>
    </html>
  );
}
