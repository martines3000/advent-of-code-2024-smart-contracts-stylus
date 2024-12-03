import { defineConfig } from '../defineConfig';

export function createDevnetConfig() {
  return defineConfig({
    env: 'devnet',
    contractAddress: '0x11673b5ef84c5e30898246fbf5499a6080ce50ca',
  });
}
