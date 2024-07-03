import { useRouter } from 'next/router';
import { MainLayout } from '@/app/main-layout';


export default function Page() {
  const router = useRouter();
  return (
    <MainLayout>
      <div>
      Thread {JSON.stringify(router.query)}
      </div>
    </MainLayout>
  );
}
