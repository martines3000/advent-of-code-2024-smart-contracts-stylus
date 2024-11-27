'use client';

import { ThemeProvider as NextThemesProvider } from 'next-themes';
import type * as React from 'react';

import '@rainbow-me/rainbowkit/styles.css';
import { WagmiProvider } from 'wagmi';
import { config } from '@/lib/wagmi/config';

export const Providers = ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  return (
    <WagmiProvider config={config}>
      <NextThemesProvider attribute="class" defaultTheme="dark">
        {children}
      </NextThemesProvider>
    </WagmiProvider>
  );
};
