import {saveNotificationSound} from '../invokes';
import {t} from '../i18n';

export const soundLabel = (value?: string | null) => {
  const trimmed = value?.trim();
  if (!trimmed) return t('common.default');
  const normalized = trimmed.replace(/\\+/g, '\\');
  const parts = normalized.split(/[/\\]/);
  return parts[parts.length - 1] || trimmed;
};

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
