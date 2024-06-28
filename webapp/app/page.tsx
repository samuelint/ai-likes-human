import Assistant from './_components/assistant';
import TauriDebug from './_components/tauri-debug';


export default function Home() {
  return (
    <main className="w-full h-full flex flex-col justify-between">
      <Assistant />
      <TauriDebug />
    </main>
  );
}
