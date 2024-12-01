import { defineConfig } from '../defineConfig';

export function createDevnetConfig() {
  return defineConfig({
    env: 'devnet',
    contractAddress: '0x75e0e92a79880bd81a69f72983d03c75e2b33dc8',
  });
}
