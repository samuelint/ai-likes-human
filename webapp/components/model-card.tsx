import { Card, CardDescription, CardHeader, CardTitle } from './ui/card';


interface Props {
  model: string
  vendor: string
  metadata?: Record<string, string>
  className?: string
}

export function ModelCard({ model, vendor, metadata = {}, className }: Props) {
  const title = Object.keys(metadata).map((key) => (
    `${key}: ${metadata[key]}`
  )).join('\n');

  return (
    <Card className={className} title={title}>
      <CardHeader>
        <CardTitle>{ model }</CardTitle>
        <CardDescription>{ vendor }</CardDescription>
      </CardHeader>
    </Card>
  );
}
