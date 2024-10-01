import { ReactNode } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';


interface Props {
  id?: string
  title: ReactNode
  description?: ReactNode
  children?: ReactNode
}

export function Section({ id, title, description, children }: Props) {
  return (
    <Card id={id}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        { description && <CardDescription>{description}</CardDescription> }
      </CardHeader>
      <CardContent className='flex flex-col gap-4'>
        { children }
      </CardContent>
    </Card>
  );
}
