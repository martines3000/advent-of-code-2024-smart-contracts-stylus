export const AOC2024 = [
  {
    inputs: [],
    name: 'init',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'uint32',
        name: 'day',
        type: 'uint32',
      },
      {
        internalType: 'string',
        name: '_address',
        type: 'string',
      },
    ],
    name: 'setSolution',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
  {
    inputs: [
      {
        internalType: 'uint32',
        name: 'day',
        type: 'uint32',
      },
      {
        internalType: 'uint32',
        name: 'part',
        type: 'uint32',
      },
      {
        internalType: 'string',
        name: 'input',
        type: 'string',
      },
    ],
    name: 'solve',
    outputs: [
      {
        internalType: 'uint32',
        name: '',
        type: 'uint32',
      },
    ],
    stateMutability: 'nonpayable',
    type: 'function',
  },
] as const;
