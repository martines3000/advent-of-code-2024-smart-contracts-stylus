import { defineConfig } from '../defineConfig';

export function createTestnetConfig() {
  return defineConfig({
    env: 'testnet',
    contractAddress: '0x4b0834946d266a2739a82b6412367aae91f35ea4',
  });
}
