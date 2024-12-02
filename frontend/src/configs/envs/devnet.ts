import { defineConfig } from '../defineConfig';

export function createDevnetConfig() {
  return defineConfig({
    env: 'devnet',
    contractAddress: '0x2c3d95da9045d3825d3b9d17d57ba18157b08ffa',
  });
}
