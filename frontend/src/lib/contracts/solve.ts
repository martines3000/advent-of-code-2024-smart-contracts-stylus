import { appConfig } from '@/configs';
import { readContract } from '@wagmi/core';
import type { Dispatch, SetStateAction } from 'react';
import { config } from '../wagmi/config';
import { AOC2024 } from './abi';

export const solve = async (
  input: string,
  day: string,
  part: string,
  setSolution: Dispatch<SetStateAction<string>>
): Promise<void> => {
  try {
    // @ts-ignore
    const result = await readContract(config, {
      abi: AOC2024,
      functionName: `solve${day}${part}` as any,
      address: appConfig.contractAddress as `0x${string}`,
      args: [input],
    });

    setSolution(result.toString());
  } catch (error) {
    console.log(error);
  }
};
