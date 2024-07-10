/* eslint-disable @next/next/no-img-element */
'use client';
import { TakeScreenshotButton } from '@/components/take-screenshot-button';
import { useToast } from '@/components/ui/use-toast';
import { useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { AddImageAttachments } from '@/lib/image-attachment.type';


interface Props {
  addImageAttachments: AddImageAttachments
}

export function AddScreenshotToInput({ addImageAttachments }: Props) {
  const { toast } = useToast();

  const addScreenshotToInput = useCallback(() => {
    invoke<string[]>('capture_screen')
      .then((result) => {
        const imageAttachments = result
          .map((value) => `data:image/png;base64,${value}`)
          .map((base64, index) => ({ title: `${index}`, base64 }));
        addImageAttachments(imageAttachments);
      })
      .catch((error) => {
        toast({
          title: 'Screenshot error',
          description: error,
          variant: 'destructive',
        });
      });


  }, [toast, addImageAttachments]);

  return (
    <TakeScreenshotButton onClick={addScreenshotToInput} />
  );
}
