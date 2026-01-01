import {Vibrant} from 'node-vibrant/browser';
import type {Palette, Swatch} from '@vibrant/color';
import {fetchCachedImageData} from '../data/images';

export type RgbColor = [number, number, number];

const BASE_RGB: RgbColor = [46, 50, 61];
const ACCENT_ALPHA = 0.28;
const BASE_ALPHA = 0.95;
const CARD_ALPHA = 0.18;

const swatchPreference: (keyof Palette)[] = [
  'Vibrant',
  'Muted',
  'DarkVibrant',
  'LightVibrant',
  'DarkMuted',
  'LightMuted',
];

const isTauriRuntime =
  typeof window !== 'undefined' && '__TAURI__' in (window as typeof window);

const colorCache = new Map<string, RgbColor | null>();
const inflight = new Map<string, Promise<RgbColor | null>>();
const sourceCache = new Map<string, string>();

const pickSwatch = (palette: Palette): Swatch | null => {
  for (const key of swatchPreference) {
    const swatch = palette[key];
    if (swatch) return swatch;
  }
  return null;
};

const resolveImageSource = async (url: string) => {
  if (sourceCache.has(url)) {
    return sourceCache.get(url) ?? url;
  }

  let resolved = url;
  if (isTauriRuntime) {
    const dataUrl = await fetchCachedImageData(url);
    if (dataUrl) {
      resolved = dataUrl;
    }
  }

  sourceCache.set(url, resolved);
  return resolved;
};

export const clearDominantColorCache = (url?: string) => {
  if (!url) {
    colorCache.clear();
    sourceCache.clear();
    inflight.clear();
    return;
  }
  colorCache.delete(url);
  sourceCache.delete(url);
  inflight.delete(url);
};

export const getDominantColor = async (url: string) => {
  if (!url) return null;
  if (colorCache.has(url)) {
    return colorCache.get(url) ?? null;
  }
  if (inflight.has(url)) {
    return inflight.get(url) ?? null;
  }

  const task = (async () => {
    try {
      const source = await resolveImageSource(url);
      const palette = await Vibrant.from(source).getPalette();
      const swatch = pickSwatch(palette);
      const rgb = swatch ? (swatch.rgb as RgbColor) : null;
      colorCache.set(url, rgb);
      return rgb;
    } catch (error) {
      console.warn('[dominantColor] Failed to extract color', error);
      colorCache.set(url, null);
      return null;
    } finally {
      inflight.delete(url);
    }
  })();

  inflight.set(url, task);
  return task;
};

export const buildCardOverlayStyle = (rgb: RgbColor | null) => {
  if (!rgb) return {};
  const [r, g, b] = rgb;
  const accent = `rgba(${r}, ${g}, ${b}, ${ACCENT_ALPHA})`;
  const base = `rgba(${BASE_RGB.join(', ')}, ${BASE_ALPHA})`;
  return {
    backgroundColor: `rgba(${r}, ${g}, ${b}, ${CARD_ALPHA})`,
    backgroundImage: `linear-gradient(180deg, ${accent}, ${base})`,
  };
};
