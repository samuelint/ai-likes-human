import NewThreadPrompt from './_components/new-thread-prompt';
import RecentThreads from './_components/recent-threads';
import { OnBoardingModal } from './_onboarding/modal';


export default function Home() {
  return (
    <main className="h-full flex flex-col">
      <RecentThreads />
      <NewThreadPrompt />
      <OnBoardingModal />
    </main>
  );
}
