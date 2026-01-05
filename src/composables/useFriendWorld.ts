import {computed, ref, type Ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {fetchWorld} from '../invokes';
import type {VRChat} from '../vrchat.ts';

const worldCache = new Map<string, VRChat.World | null>();
const worldRequests = new Map<string, Promise<VRChat.World | null>>();

const splitLocation = (location: string) => {
  if (!location.startsWith('wrld_')) return {worldId: null, instanceId: ''};
  const [worldId, instanceId = ''] = location.split(':');
  return {
    worldId: worldId || null,
    instanceId,
  };
};

const toInstanceShortId = (instanceId: string) => {
  if (!instanceId) return '';
  const [shortId] = instanceId.split('~');
  return shortId ?? '';
};

const loadWorld = async (worldId: string) => {
  if (worldCache.has(worldId)) {
    return worldCache.get(worldId) ?? null;
  }
  if (!worldRequests.has(worldId)) {
    worldRequests.set(
      worldId,
      fetchWorld(worldId)
        .then((world) => {
          worldCache.set(worldId, world ?? null);
          return world ?? null;
        })
        .catch((error) => {
          console.error(error);
          worldCache.set(worldId, null);
          return null;
        })
        .finally(() => {
          worldRequests.delete(worldId);
        }),
    );
  }
  return worldRequests.get(worldId) ?? null;
};

export const useFriendWorld = (friend: Ref<VRChat.LimitedUserFriend>) => {
  const {t} = useI18n();
  const world = ref<VRChat.World | null>(null);

  const locationParts = computed(() => splitLocation(friend.value.location ?? ''));
  const worldId = computed(() => locationParts.value.worldId);
  const instanceId = computed(() => locationParts.value.instanceId);
  const instanceShortId = computed(() => toInstanceShortId(locationParts.value.instanceId));
  const isWorldLocation = computed(() => Boolean(worldId.value));

  watch(
    worldId,
    async (nextWorldId) => {
      if (!nextWorldId) {
        world.value = null;
        return;
      }
      world.value = await loadWorld(nextWorldId);
    },
    {immediate: true},
  );

  const worldImageUrl = computed(() =>
    world.value?.imageUrl || world.value?.thumbnailImageUrl || '',
  );

  const locationLabel = computed(() => {
    const raw = friend.value.location ?? '';
    if (!raw) return '';
    const normalized = raw.toLowerCase();
    if (normalized.startsWith('wrld_')) {
      if (world.value?.name) {
        return instanceShortId.value
          ? `${world.value.name} #${instanceShortId.value}`
          : world.value.name;
      }
      return raw;
    }
    if (normalized === 'private') return t('friends.location.private');
    if (normalized === 'traveling') return t('friends.location.traveling');
    if (normalized === 'offline') return t('friends.location.offline');
    if (normalized === 'web') return t('friends.location.web');
    return raw;
  });

  return {
    isWorldLocation,
    instanceId,
    worldId,
    locationLabel,
    worldImageUrl,
  };
};
