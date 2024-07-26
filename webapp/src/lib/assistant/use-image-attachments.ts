import { useState } from 'react';
import { ImageAttachment } from './image-attachment.type';


export function useImageAttachments() {
  const [imageAttachments, setImageAttachments] = useState<ImageAttachment[]>([]);

  const addImageAttachments = (newImageAttachments: ImageAttachment[]) => {
    setImageAttachments(imageAttachments => [...imageAttachments, ...newImageAttachments]);
  };

  const removeImageAttachment = (imageAttachment: ImageAttachment) => {
    setImageAttachments(imageAttachments => imageAttachments.filter(img => img !== imageAttachment));
  };

  return { imageAttachments, setImageAttachments, addImageAttachments, removeImageAttachment };
}