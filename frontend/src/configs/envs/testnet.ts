import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x649e50102afb44c5989ca4312ca53aa21db1ab66',
  });
}
