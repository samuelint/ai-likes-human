
import { useUsername } from './use-username';


export default function BrandLink() {
  const userName = useUsername();
  const title = userName ? `${userName}'s Assistant` : 'Assistant';

  return (
    <a href="/">
      <span>{title}</span>
    </a>
  );
}
