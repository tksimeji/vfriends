import {Temporal} from '@js-temporal/polyfill';
import {computed, type Ref} from 'vue';
import {t} from '../i18n';
import type {VRChat} from '../vrchat.ts';

export const isOffline = (friend: VRChat.LimitedUserFriend) => {
  const status = friend.status.toLowerCase();
  const location = friend.location.toLowerCase();
  return status === 'offline' || location === 'offline' || location === 'web';
};

export const resolveUserStatus = (
  friend: VRChat.LimitedUserFriend,
): VRChat.UserStatus => (isOffline(friend) ? 'offline' : friend.status);

export const formatLastOnline = (friend: VRChat.LimitedUserFriend) => {
  if (!isOffline(friend)) return null;

  const rawValue =
    friend.last_activity ??
    (friend as { lastActivity?: string }).lastActivity ??
    friend.last_login ??
    (friend as { lastLogin?: string }).lastLogin;

  if (!rawValue) return null;

  try {
    const lastOnline = Temporal.Instant.from(rawValue);
    const now = Temporal.Now.instant();
    const diff = now.since(lastOnline, {largestUnit: 'hour'});
    const diffMinutes = Math.floor(diff.total({unit: 'minute'}));

    if (diffMinutes < 60) {
      return t('time.minutesAgo', {count: diffMinutes});
    }

    const diffHours = Math.floor(diff.total({unit: 'hour'}));
    if (diffHours < 24) {
      return t('time.hoursAgo', {count: diffHours});
    }

    const diffDays = Math.floor(diffHours / 24);
    return t('time.daysAgo', {count: diffDays});
  } catch {
    return null;
  }
};

export const useFriendStatus = (friend: Ref<VRChat.LimitedUserFriend>) => {
  const offline = computed(() => isOffline(friend.value));
  const status = computed(() => resolveUserStatus(friend.value));
  const lastOnline = computed(() => formatLastOnline(friend.value));

  return {
    isOffline: offline,
    status,
    lastOnline,
  };
};
