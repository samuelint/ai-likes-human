import { ExtensionInfoDto } from '@/lib/extension/extension.dto';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { ReactNode } from 'react';

interface Props {
  extension: ExtensionInfoDto
  children?: ReactNode
}

export function ExtensionCard({ extension, children }: Props) {

  return (
    <Card>
      <CardHeader>
        <CardTitle>{ extension.name }</CardTitle>
        <CardDescription>{ extension.description }</CardDescription>
      </CardHeader>
      <CardContent>
        <ul className='list-inside list-none text-xs text-slate-400'>
          <li>Version: { extension.version }</li>
          <li>Author: { extension.author }</li>
        </ul>

      </CardContent>
      { children && <CardFooter className="flex gap-2 justify-between">
        { children }
      </CardFooter> }
    </Card>
  );
}
