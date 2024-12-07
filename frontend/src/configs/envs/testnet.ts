import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x765fb73552ab1514b30d81e150ba82a056ae2d3e',
  });
}
