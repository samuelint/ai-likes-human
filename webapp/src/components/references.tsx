import { MessageReference } from '@/lib/message-reference.type';
import { AppLink } from './app-link';


interface Props {
  references: MessageReference[]
}

export function References({ references }: Props) {
  return (
    <section className='flex flex-row text-xs gap-2 text-top pb-1 overflow-x-auto'>
      { references.map((reference, index) => (
        <AppLink className='text-end' href={reference.link} key={index}>{ reference.title || reference.link }</AppLink>
      ))}
    </section>
  );
}
