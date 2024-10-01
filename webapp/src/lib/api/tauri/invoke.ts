import { invoke as tauriInvoke, InvokeArgs } from '@tauri-apps/api/tauri';

export async function invoke<T>(cmd: string, args?: InvokeArgs): Promise<T> {
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (e) {
    throw Error(`${e}`);
  }
}