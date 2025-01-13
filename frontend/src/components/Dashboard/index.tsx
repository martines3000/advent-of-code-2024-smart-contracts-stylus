'use client';

import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { solve } from '@/lib/contracts';
import { Label } from '@radix-ui/react-label';
import React, { useState } from 'react';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select';

const SOLVED_PROBLEMS = [
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
  23, 24,
];
const SKIP_PART_1 = [0, 17, 22];
const SKIP_PART_2 = [6, 7, 9, 11, 16, 18, 20, 22, 23, 24];

export const Dashboard = () => {
  const [input, setInput] = useState('');
  const [solution, setSolution] = useState('');
  const [selectedProblem, setSelectedProblem] = useState<string | undefined>(
    undefined
  );

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
          value={input}
          onChange={(e) => setInput(e.target.value)}
        />
      </div>
      <div className="sm:h-full flex flex-col justify-center gap-2">
        <Button
          disabled={!input || selectedProblem === undefined}
          onClick={async () => {
            const [day, part] = selectedProblem!.split('-');
            await solve(input, day, part, setSolution);
          }}
        >
          Solve
        </Button>
        <Select value={selectedProblem} onValueChange={setSelectedProblem}>
          <SelectTrigger className="sm:w-[180px]">
            <SelectValue placeholder="Problem" />
          </SelectTrigger>
          <SelectContent>
            {SOLVED_PROBLEMS.map((problem) => (
              <React.Fragment key={problem}>
                {SKIP_PART_1.includes(problem) ? null : (
                  <SelectItem key={`${problem}-1`} value={`${problem}-1`}>
                    Day {problem} Part 1
                  </SelectItem>
                )}
                {SKIP_PART_2.includes(problem) ? null : (
                  <SelectItem key={`${problem}-2`} value={`${problem}-2`}>
                    Day {problem} Part 2
                  </SelectItem>
                )}
              </React.Fragment>
            ))}
          </SelectContent>
        </Select>
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
