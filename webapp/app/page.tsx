import Assistant from './_components/assistant';
import RecentThreads from './_components/recent-threads';


export default function Home() {
  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      <Assistant />
    </main>
  );
}
