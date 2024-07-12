import { Alert, AlertDescription } from './alert';


interface Props {
  error?: Error
}

export function ErrorDetails({ error }: Props) {
  return error && (
    <Alert variant="destructive" className='px-4 py-2'>
      <AlertDescription>
        {error?.message}
      </AlertDescription>
    </Alert>
  );
}

