import { createDevnetConfig } from './envs/devnet';
import { createTestnetConfig } from './envs/testnet';

export const appConfig = getConfig();

function getConfig() {
  const appEnv = process.env.NEXT_PUBLIC_APP_ENV ?? 'devnet';
  switch (appEnv) {
    case 'devnet':
      return createDevnetConfig();
    case 'testnet':
      return createTestnetConfig();
    default:
      throw new Error(`Invalid APP_ENV "${process.env.NEXT_PUBLIC_APP_ENV}"`);
  }
}
