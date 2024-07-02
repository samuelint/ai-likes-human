
interface Props {
  error?: Error
}

export function FormatError({ error }: Props) {
  return error && (
    <span className='p-2 text-xs text-destructive'>{error?.message}</span>
  );
}
