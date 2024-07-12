
interface Props {
  model: string
}

export function Model({ model }: Props) {
  const [vendor, name] = model.split(':');

  return (
    <div><span className='capitalize font-bold'>{vendor}</span> - <span>{name}</span></div>
  );
}
