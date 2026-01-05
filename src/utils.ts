import {saveNotificationSound} from './invokes.ts';

export const resolveSoundPath = async (file: File | null) => {
  if (!file) return null;
  const directPath = (file as File & { path?: string }).path;
  if (directPath && !directPath.includes('fakepath')) {
    return directPath;
  }

  const buffer = await file.arrayBuffer();
  const bytes = Array.from(new Uint8Array(buffer));
  const storedPath = await saveNotificationSound(file.name, bytes);
  return storedPath || null;
};
