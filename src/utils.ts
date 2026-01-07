import {saveNotificationSound, saveNotificationSoundPath} from './invokes.ts';

const yieldToUi = () =>
  new Promise<void>((resolve) => {
    if (typeof requestAnimationFrame === 'function') {
      requestAnimationFrame(() => resolve());
    } else {
      setTimeout(resolve, 0);
    }
  });

const bytesFromBuffer = async (buffer: ArrayBuffer) => {
  const bytes = new Uint8Array(buffer);
  const result = new Array<number>(bytes.length);
  const chunkSize = 4096;
  for (let index = 0; index < bytes.length; index += 1) {
    result[index] = bytes[index];
    if (index % chunkSize === 0) {
      await yieldToUi();
    }
  }
  return result;
};

export const resolveSoundPath = async (file: File | null) => {
  if (!file) return null;
  const directPath = (file as File & { path?: string }).path;
  if (directPath && !directPath.includes('fakepath')) {
    const storedPath = await saveNotificationSoundPath(directPath);
    return storedPath || null;
  }

  const buffer = await file.arrayBuffer();
  await yieldToUi();
  const bytes = await bytesFromBuffer(buffer);
  const storedPath = await saveNotificationSound(file.name, bytes);
  return storedPath || null;
};
