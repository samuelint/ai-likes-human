/* eslint-disable @next/next/no-img-element */
'use client';
import { TakeScreenshotButton } from '@/components/take-screenshot-button';
import { useToast } from '@/components/ui/use-toast';
import { useCallback, useState } from 'react';
import { AddImageAttachments } from '@/lib/image-attachment.type';
import { captureBase64Screens } from '@/lib/tauri-interrupt/screen-capture';
import { SpinnerLoading } from '@/components/ui/loading';


interface Props {
  addImageAttachments: AddImageAttachments
}

export function AddScreenshotToInput({ addImageAttachments }: Props) {
  const { toast } = useToast();
  const [isLoading, setIsLoading] = useState(false);

  const addScreenshotToInput = useCallback(async () => {
    try {
      setIsLoading(true);
      const base64Images = await captureBase64Screens();
      base64Images.map((base64, index) => {
        const imageAttachments = [{ title: `${index}`, base64 }];
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

  return (<>
    {isLoading ? <SpinnerLoading /> : <TakeScreenshotButton onClick={addScreenshotToInput} />}
  </>);
}

