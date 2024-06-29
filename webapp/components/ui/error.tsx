
interface Props {
  error?: Error
}

export function FormatError({ error }: Props) {
  return error && (
    <span className='text-destructive'>{error?.message}</span>
  );
}

