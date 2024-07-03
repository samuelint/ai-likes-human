import AssistantThread from './_components/assistant-thread';
import RecentThreads from './_components/recent-threads';


export default function Home() {
  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      <AssistantThread />
    </main>
  );
}
