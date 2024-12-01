import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x782ebfb915b318d0bf2109d0809c63ca059b08a4',
  });
}
