'use client';

import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@radix-ui/react-label';
import { useState } from 'react';

export const Dashboard = () => {
  const [input, setInput] = useState('');
  const [solution, setSolution] = useState('asdfa sdfas dfsadfasd');

  const handleClick = () => {
    console.log('clicked');
  };

  return (
    <>
      <div className="flex flex-col flex-1 gap-1.5 p-1 overflow-hidden">
        <Label htmlFor="input" className="font-semibold text-lg">
          Input
        </Label>
        <Textarea
          id="input"
          placeholder="Input goes here."
          className="h-full w-full resize-none"
        />
      </div>
      <div className="flex flex-wrap gap-2 w-full">
        <Button
          variant="default"
          onMouseDown={() => {
            handleClick();
          }}
        >
          Solve 1
        </Button>
      </div>
      <div className="flex flex-col flex-1 gap-1.5 p-1 overflow-hidden">
        <Label htmlFor="solution" className="font-semibold text-lg">
          Solution
        </Label>
        <Textarea
          id="solution"
          disabled
          placeholder="Solution will be displayed here."
          className="h-full w-full resize-none"
          value={solution}
        />
      </div>
    </>
  );
};
