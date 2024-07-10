
interface Props {
  base64: string
  alt?: string
  className?: string
}

export function Base64Image({ base64, alt, className }: Props) {
  return (
    // eslint-disable-next-line @next/next/no-img-element
    <img className={className} src={base64} alt={alt} />
  );
}
