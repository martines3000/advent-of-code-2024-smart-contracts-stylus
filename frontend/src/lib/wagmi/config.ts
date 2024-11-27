import { appConfig } from '@/configs';
import { arbitrumSepolia } from 'viem/chains';
import { createConfig, http } from 'wagmi';

export const localDevNodeArbitrum = {
  id: 412346,
  name: 'Localhost',
  nativeCurrency: {
    decimals: 18,
    name: 'Ether',
    symbol: 'ETH',
  },
  rpcUrls: {
    default: { http: ['http://localhost:8547'] },
  },
};

export const config = createConfig({
  ssr: false,
  chains: [appConfig.env === 'devnet' ? localDevNodeArbitrum : arbitrumSepolia],
  transports: {
    ...(appConfig.env === 'devnet'
      ? {
          [localDevNodeArbitrum.id]: http('http://localhost:8547'),
        }
      : {
          [arbitrumSepolia.id]: http(),
        }),
  },
});
