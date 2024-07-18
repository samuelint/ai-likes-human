import { MessageReference } from '@/lib/message-reference.type';
import { AppLink } from './app-link';


interface Props {
  references: MessageReference[]
}

export function References({ references }: Props) {
  return (
    <section
      className='flex flex-row text-xs gap-2 text-top pb-1 overflow-x-auto'
      style={{
        whiteSpace: 'nowrap',
        wordBreak: 'keep-all',
        textOverflow: 'clip',
      }}
    >
      { references.map((reference, index) => (
        <AppLink
          className='text-end'
          href={reference.link}
          key={index}
          style={{
            maxHeight: '1rem',
            lineHeight: '1rem',
          }}
        >
          { reference.title || reference.link }
        </AppLink>
      ))}
    </section>
  );
}
