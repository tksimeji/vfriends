import {invoke} from '@tauri-apps/api/core';

export const fetchCachedImageData = async (url: string) => {
  if (!url) return null;
  return invoke<string | null>('fetch_cached_image_data', {url});
};
