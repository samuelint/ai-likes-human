/* eslint-disable @next/next/no-img-element */
'use client';
import { TakeScreenshotButton } from '@/components/take-screenshot-button';
import { useToast } from '@/components/ui/use-toast';
import { useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';


interface Props {

}

export function AddScreenshotToInput({ }: Props) {
  const { toast } = useToast();


  const addScreenshotToInput = useCallback(() => {
    invoke<string[]>('capture_screen', { name: 'Next.js' })
      .then((result) => {
        console.log(result);
        toast({
          title: 'Add screenshot added as context',
          description: (<>
            { result.map((value, index) => <img key={index} src={`data:image/png;base64,${value}`} alt="Base64 Image" />) }
          </>)
        });
      })
      .catch((error) => {
        toast({
          title: 'Screenshot error',
          description: error,
          variant: 'destructive',
        });
      });


  }, [toast]);

  return (
    <TakeScreenshotButton onClick={addScreenshotToInput} />
  );
}
