import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { ExtensionStateDto } from '@/lib/extension/extension.dto';
import { PropsWithChildren, ReactNode } from 'react';

export type Actions<T extends Extensions = Extensions> = (extension: T) => ReactNode;
type Extensions = Pick<ExtensionStateDto, 'name' | 'version' | 'ipc_port'>;
interface Props<T extends Extensions = Extensions> extends PropsWithChildren {
  extensions?: T[]
  Actions?: Actions<T>
}

export function ExtensionTable({ extensions, Actions, children }: Props) {

  return (
    <Table className='w-full'>
      <TableCaption>
        { children && <div className='p-4'>{children}</div>}
      </TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead className="w-[160px]">Name</TableHead>
          <TableHead className="w-[100px]">Version</TableHead>
          <TableHead className="w-[100px]">Port</TableHead>
          <TableHead className="text-right"></TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        { extensions?.map((extension) => (
          <TableRow key={extension.name}>
            <TableCell className="font-medium">{extension.name}</TableCell>
            <TableCell>{extension.version}</TableCell>
            <TableCell>{extension.ipc_port}</TableCell>
            <TableCell className="text-right flex justify-end gap-2">
              { Actions && Actions(extension) }
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
