import AssistantThread from '@/app/_components/assistant-thread';
import RecentThreads from '@/app/_components/recent-threads';


interface Props {
  threadId?: string
}

export function Thread({ threadId }: Props) {

  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      { threadId && <AssistantThread threadId={threadId} /> }
    </main>
  );
}
