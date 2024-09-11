import { useToast } from '@/components/ui/use-toast';
import { assertScreenCapturePermissions } from '@/lib/tauri-command/screen-capture';
import { useMount } from 'react-use';

export function useAssertScreenCapturePermissions() {
  const { toast } = useToast();

  useMount(async () => {
    try {
      await assertScreenCapturePermissions();
    } catch (e) {
      toast({
        title: 'Screen capture permission error',
        description: e?.toString(),
        variant: 'destructive'
      });
    }
  });
}
