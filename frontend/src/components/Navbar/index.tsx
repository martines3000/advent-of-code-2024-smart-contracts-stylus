import { Nunito } from 'next/font/google';
import { cn } from '@/lib/utils';

const nunito = Nunito({ subsets: ['latin'] });

export const Navbar = () => {
  return (
    <header className="w-full pt-8 px-6">
      <nav className="w-full pb-4 text-center">
        <div className={cn('font-bold text-xl', nunito.className)}>
          AOC 2024 USING SMART CONTRACTS
        </div>
      </nav>
    </header>
  );
};
