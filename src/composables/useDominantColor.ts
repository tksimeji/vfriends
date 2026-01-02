import {computed, ref, unref, watch, type ComputedRef, type Ref} from 'vue';
import {VRChat} from '../vrchat.ts';

type DominantSource = VRChat.UserSummary | string | null | undefined;

const resolveSourceUrl = (value: DominantSource) => {
  if (!value) return '';
  if (typeof value === 'string') return value;
  return VRChat.resolveAvatarUrl(value);
};

export const useDominantColor = (
  source: Ref<DominantSource> | ComputedRef<DominantSource>,
) => {
  const rgb = ref<VRChat.RgbColor | null>(null);
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
      const value = sourceValue.value;
      const color =
        typeof value === 'string'
          ? await VRChat.dominantColorForUrl(next)
          : value
            ? await VRChat.dominantColorForUser(value)
            : null;
      if (currentId !== requestId) return;

      rgb.value = color;
      isLoading.value = false;
    },
    {immediate: true},
  );

  const overlayStyle = computed(() => VRChat.cardOverlayStyle(rgb.value));

  return {
    rgb,
    overlayStyle,
    isLoading,
  };
};
