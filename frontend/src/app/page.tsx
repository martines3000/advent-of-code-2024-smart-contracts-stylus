import { Dashboard } from '@/components/dashboard';

export default function Home() {
  return (
    <div className="items-center justify-items-center min-h-screen h-screen p-8 font-[family-name:var(--font-geist-sans)]">
      <main className="h-full w-full">
        <div className="h-full w-full flex flex-col gap-4">
          <h1 className="font-bold text-2xl text-center text-primary">
            AOC 2024
          </h1>
          <Dashboard />
        </div>
      </main>
    </div>
  );
}
