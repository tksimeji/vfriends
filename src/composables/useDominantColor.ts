import {computed, ref, unref, watch, type ComputedRef, type Ref} from 'vue';
import {Vibrant} from 'node-vibrant/browser';
import type {Palette, Swatch} from '@vibrant/color';
import {fetchIconDataUri} from '../invokes.ts';
import {VRChat} from '../vrchat.ts';

type RgbColor = [number, number, number];

type DominantSource = VRChat.EitherFriendOrCurrent | string | null | undefined;

const resolveSourceUrl = (value: DominantSource) => {
  if (!value) return '';
  if (typeof value === 'string') return value;
  return VRChat.resolveAvatarUrl(value);
};

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

const BASE_RGB: RgbColor = [46, 50, 61];
const ACCENT_ALPHA = 0.28;
const BASE_ALPHA = 0.95;
const CARD_ALPHA = 0.18;

const colorCache = new Map<string, RgbColor | null>();
const inflight = new Map<string, Promise<RgbColor | null>>();
const sourceCache = new Map<string, string>();

const dominantColorForUrl = async (url: string) => {
  if (!url) return null;
  if (colorCache.has(url)) {
    return colorCache.get(url) ?? null;
  }
  if (inflight.has(url)) {
    return inflight.get(url) ?? null;
  }

  const task = (async () => {
    try {
      let source = url;
      if (sourceCache.has(url)) {
        source = sourceCache.get(url) ?? url;
      } else if (isTauriRuntime) {
        const dataUrl = await fetchIconDataUri(url);
        if (dataUrl) {
          source = dataUrl;
        }
        sourceCache.set(url, source);
      }

      const palette = await Vibrant.from(source).getPalette();
      let swatch: Swatch | null = null;
      for (const key of swatchPreference) {
        const candidate = palette[key];
        if (candidate) {
          swatch = candidate;
          break;
        }
      }
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

const cardOverlayStyle = (rgb: RgbColor | null) => {
  if (!rgb) return {};
  const [r, g, b] = rgb;
  const accent = `rgba(${r}, ${g}, ${b}, ${ACCENT_ALPHA})`;
  const base = `rgba(${BASE_RGB.join(', ')}, ${BASE_ALPHA})`;
  return {
    backgroundColor: `rgba(${r}, ${g}, ${b}, ${CARD_ALPHA})`,
    backgroundImage: `linear-gradient(180deg, ${accent}, ${base})`,
  };
};

export const useDominantColor = (
  source: Ref<DominantSource> | ComputedRef<DominantSource>,
) => {
  const rgb = ref<RgbColor | null>(null);
  const isLoading = ref(false);
  let requestId = 0;

  const sourceValue = computed(() => unref(source));
  const sourceKey = computed(() => resolveSourceUrl(sourceValue.value));

  watch(
    sourceKey,
    async (next) => {
      requestId += 1;
      const currentId = requestId;

      if (!next) {
        rgb.value = null;
        isLoading.value = false;
        return;
      }

      isLoading.value = true;
      const color = await dominantColorForUrl(next);
      if (currentId !== requestId) return;

      rgb.value = color;
      isLoading.value = false;
    },
    {immediate: true},
  );

  const overlayStyle = computed(() => cardOverlayStyle(rgb.value));

  return {
    rgb,
    overlayStyle,
    isLoading,
  };
};
