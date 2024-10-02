import { TakeScreenshotButton } from '@/components/take-screenshot-button';
import { useToast } from '@/components/ui/use-toast';
import { useCallback, useState } from 'react';
import { AddImageAttachments } from '@/lib/assistant/image-attachment.type';
import { captureBase64Screens } from '@/lib/core-api/tauri/screen-capture';
import { useAssertScreenCapturePermissions } from './hook/use-assert-screen-capture-permissions';


interface Props {
  addImageAttachments: AddImageAttachments
}

export function AddScreenshotToInput({ addImageAttachments }: Props) {
  const { toast } = useToast();
  const [isLoading, setIsLoading] = useState(false);

  useAssertScreenCapturePermissions();

  const addScreenshotToInput = useCallback(async () => {
    try {
      setIsLoading(true);
      const base64Images = await captureBase64Screens();
      base64Images.map((base64, index) => {
        const imageAttachments = [{ title: `Monitor ${index}`, base64 }];
        addImageAttachments(imageAttachments);
      });
    } catch (error) {
      toast({
        title: 'Screenshot error',
        description: error?.toString(),
        variant: 'destructive',
      });
    } finally {
      setIsLoading(false);
    }

  }, [toast, addImageAttachments]);

  return <TakeScreenshotButton onClick={addScreenshotToInput} isLoading={isLoading}/>;
}