import NewThreadPrompt from './_components/new-thread-prompt';
import RecentThreads from './_components/recent-threads';


export default function Home() {
  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      <NewThreadPrompt />
    </main>
  );
}
