import { cn } from '@/lib/utils';
import { Nunito } from 'next/font/google';

const nunito = Nunito({ subsets: ['latin'] });

export const Navbar = () => {
  return (
    <header className="w-full pt-8 px-6">
      <nav className="w-full pb-4 text-center">
        <div
          className={cn(
            'font-bold text-2xl pb-1 text-primary',
            nunito.className
          )}
        >
          Advent of Code 2024 - Smart Contracts
        </div>
        <p>
          Solutions are written as Smart Contracts in Rust and deployed on
          Arbitrum Sepolia thanks to{' '}
          <a
            className="underline hover:cursor-pointer text-primary"
            href="https://arbitrum.io/stylus"
          >
            Arbitrum Stylus
          </a>
        </p>
      </nav>
    </header>
  );
};
