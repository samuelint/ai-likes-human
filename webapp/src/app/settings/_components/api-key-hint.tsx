import { ExternalLink } from '@/components/external-link';


interface Props {
  how_to_get_api_key_url?: string
}

export default function ApiKeysHint({ how_to_get_api_key_url }: Props) {
  return (
    <>
      { how_to_get_api_key_url && <span><ExternalLink href={how_to_get_api_key_url}>{how_to_get_api_key_url}</ExternalLink></span>}
    </>
  );
}
