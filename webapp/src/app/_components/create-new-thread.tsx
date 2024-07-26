import { NewThreadButton } from '@/components/new-thread-button';
import { useCreateThread } from '@/lib/assistant/use-create-thread';
import { NewThreadContextMenu } from './new-thread-context-menu';

export function CreateNewThread() {
  const createThread = useCreateThread({ redirect: true });

  return (
    <NewThreadContextMenu>
      <NewThreadButton onClick={() => createThread()} />
    </NewThreadContextMenu>
  );
}
