import { MessageReference } from '@/lib/assistant/message-reference.type';
import { ExternalLink } from './external-link';


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
        <ExternalLink
          className='text-end'
          href={reference.link}
          key={index}
          style={{
            maxHeight: '1rem',
            lineHeight: '1rem',
          }}
        >
          { reference.title || reference.link }
        </ExternalLink>
      ))}
    </section>
  );
}
