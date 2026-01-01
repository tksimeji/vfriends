import {computed, ref, watch, type ComputedRef, type Ref} from 'vue';
import {buildCardOverlayStyle, getDominantColor, type RgbColor} from '../ui/dominantColor';

export const useDominantColor = (source: Ref<string> | ComputedRef<string>) => {
  const rgb = ref<RgbColor | null>(null);
  const isLoading = ref(false);
  let requestId = 0;

  watch(
    source,
    async (next) => {
      requestId += 1;
      const currentId = requestId;

      if (!next) {
        rgb.value = null;
        isLoading.value = false;
        return;
      }

      isLoading.value = true;
      const color = await getDominantColor(next);
      if (currentId !== requestId) return;

      rgb.value = color;
      isLoading.value = false;
    },
    {immediate: true},
  );

  const overlayStyle = computed(() => buildCardOverlayStyle(rgb.value));

  return {
    rgb,
    overlayStyle,
    isLoading,
  };
};
