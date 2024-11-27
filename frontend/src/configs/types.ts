import { z } from 'zod';

export const AppConfigSchema = z.object({
  env: z.enum(['devnet', 'testnet']),
  contractAddress: z.string(),
});

export type AppConfig = z.infer<typeof AppConfigSchema>;
