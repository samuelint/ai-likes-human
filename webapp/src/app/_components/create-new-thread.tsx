
import { NewThread } from '@/components/new-thread';
import { useCreateThread } from '@/lib/use-create-thread';



export function CreateNewThread() {
  const createThread = useCreateThread({ redirect: true });

  return (
    <NewThread onClick={() => createThread()} />
  );
}
