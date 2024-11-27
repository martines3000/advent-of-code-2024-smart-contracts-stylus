import { defineConfig } from '../defineConfig';

export function createDevnetConfig() {
  return defineConfig({
    env: 'devnet',
    contractAddress: '0x85d9a8a4bd77b9b5559c1b7fcb8ec9635922ed49',
  });
}
