import { z } from 'zod';

export interface ExtensionInfoDto {
  name: string
  description?: string
  version: string
  author: string
}

export const UploadExtensionSchema = z.object({
  file: z
    .instanceof(File, { message: 'Please upload a .whl file' })
    .refine((file) => {
      return file.name.endsWith('.whl');})
});
export type UploadExtensionDto = z.infer<typeof UploadExtensionSchema>;