import { z } from 'zod';

export interface ExtensionStateDto {
  name: string
  description?: string
  version: string
  author: string

  status: 'installed' | 'loaded'

  pid?: number
  ipc_port?: number
}
export const ACCEPTED_EXTENSIONS_FILE_EXTENSION = ['.pex'] as const;

export const UploadExtensionSchema = z.object({
  file: z
    .instanceof(File, { message: 'Please upload a .whl file' })
    .refine((file) => ACCEPTED_EXTENSIONS_FILE_EXTENSION.some(ext => file.name.endsWith(ext)))
});
export type UploadExtensionDto = z.infer<typeof UploadExtensionSchema>;