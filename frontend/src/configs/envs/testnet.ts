import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x98b9d4881c9b5d743891cafee3def5230c3d0ee3',
  });
}
