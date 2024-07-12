export interface ImageAttachment {
  title: string
  base64: string
}

export type AddImageAttachments = (imageAttachment: ImageAttachment[]) => void;