import { ExtensionInfoDto } from '@/lib/extension/extension.dto';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';

interface Props {
  extension: ExtensionInfoDto
}

export function ExtensionCard({ extension }: Props) {

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
      <CardFooter className="flex justify-between">
        <Button variant="destructive">Delete</Button>
      </CardFooter>
    </Card>
  );
}
