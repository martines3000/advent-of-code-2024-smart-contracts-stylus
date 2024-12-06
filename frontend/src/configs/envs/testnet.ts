import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x65d09ca88bb604c5cc5e5c33172b9f7b896fc2a2',
  });
}
