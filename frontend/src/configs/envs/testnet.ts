import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0xed08bbab7b97525c94d9d6c436a93e2060dbb995',
  });
}
