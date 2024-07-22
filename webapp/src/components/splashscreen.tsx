import { HelixLoading } from '@/components/ui/loading';

export default function Splashscreen() {
  return (
    <div className="flex flex-col items-center justify-center gap-6 fixed inset-0 backdrop-filter backdrop-blur-sm z-50">
      <div className='flex flex-col'>
        <h1 className='text-3xl font-medium tracking-tighter'>Launching Assistant</h1>
        <p>Please wait</p>
      </div>
      <HelixLoading />
    </div>
  );

}
