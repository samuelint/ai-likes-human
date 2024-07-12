import { Link } from 'wouter';
import { useUsername } from './use-username';


export default function BrandLink() {
  const userName = useUsername();
  const title = userName ? `${userName}'s Assistant` : 'Assistant';

  return (
    <Link href="/">
      <span>{title}</span>
    </Link>
  );
}
