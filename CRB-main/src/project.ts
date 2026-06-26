import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import type { BearingInput } from './types/bearing';

export const PROJECT_VERSION = 1;

export interface ProjectFile {
  version: number;
  created: string;
  input: BearingInput;
}

/**
 * Open a .trb.json file and return the parsed BearingInput.
 * Returns null if the user cancels.
 */
export async function openProjectFile(): Promise<BearingInput | null> {
  const path = await open({
    title: 'Open Project',
    filters: [{ name: 'TRB Project', extensions: ['trb.json', 'json'] }],
    multiple: false,
    directory: false,
  });
  if (!path) return null;

  const text = await readTextFile(path);
  const data = JSON.parse(text);

  // Support both wrapped (ProjectFile) and raw (BearingInput) formats
  if (data.version && data.input) {
    return data.input as BearingInput;
  }
  return data as BearingInput;
}
