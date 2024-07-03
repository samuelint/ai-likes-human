import { Alert, AlertDescription } from './alert';


interface Props {
  error?: Error
}

export function ErrorDetails({ error }: Props) {
  return error && (
    <Alert variant="destructive">
      <AlertDescription>
        {error?.message}
      </AlertDescription>
    </Alert>
  );
}

