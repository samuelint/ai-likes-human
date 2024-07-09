import { Run } from 'openai/resources/beta/threads/runs/runs.mjs';


interface Props {
  run: Pick<Run, 'model'>
}

export function MessageRunDetails({ run }: Props) {
  return (
    <div className='font-light text-slate-400 text-xs select-none'>
      <span>{run.model}</span>
    </div>
  );
}
