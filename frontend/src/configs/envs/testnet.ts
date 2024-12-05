import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x2931b1d359401a2a45aaceb7ac9f368c20f6564a',
  });
}
