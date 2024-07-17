import { ThreeDotsLoading } from '@/components/ui/loading';

export default function Splashscreen() {
  return (
    <div className="flex flex-col items-center justify-center gap-4 fixed inset-0 backdrop-filter backdrop-blur-sm z-50">
      <p>Loading</p>
      <ThreeDotsLoading />
    </div>
  );

}
