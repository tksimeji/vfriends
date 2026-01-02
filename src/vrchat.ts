import {Temporal} from '@js-temporal/polyfill';
import {Vibrant} from 'node-vibrant/browser';
import type {Palette, Swatch} from '@vibrant/color';
import {fetchCachedImageData} from './data/images';
import {t} from './i18n';

export namespace VRChat {
  /**
   * Represents the `models::CurrentUser` structure of the VRChat API's rust SDK.
   */
  export type CurrentUser = {
    currentAvatarThumbnailImageUrl?: string;
    currentAvatarImageUrl: string;
    displayName: string;
    id: string;
    imageUrl?: string;
    profilePicOverride: string;
    profilePicOverrideThumbnail?: string;
    username?: string;
    userIcon: string;
  };

  /**
   * Represents the `models::LimitedUserFriend` structure of the VRChat API's rust SDK.
   */
  export type LimitedUserFriend = {
    bio?: string;
    bioLinks?: string[];
    currentAvatarImageUrl?: string;
    currentAvatarTags?: string[];
    currentAvatarThumbnailImageUrl?: string;
    developerType: DeveloperType;
    displayName: string;
    friendKey: string;
    id: string;
    imageUrl: string;
    isFriend: boolean;
    last_activity?: string;
    last_login?: string;
    last_mobile?: string;
    last_platform: string;
    location: string;
    platform: string;
    profilePicOverride?: string;
    profilePicOverrideThumbnail?: string;
    status: UserStatus;
    statusDescription: string;
    tags: string[];
    userIcon?: string;
  };

  /**
   * Represents the `model::DeveloperType` enum of the VRChat API's rust SDK.
   */
  export type DeveloperType = 'internal' | 'moderator' | 'none' | 'trusted';

  /**
   * Represents the `model::UserStatus` enum of the VRChat API's rust SDK.
   */
  export type UserStatus = 'active' | 'ask me' | 'busy' | 'join me' | 'offline';

  export const isOffline = (friend: LimitedUserFriend) => {
    const status = friend.status.toLowerCase();
    const location = friend.location.toLowerCase();
    return status === 'offline' || location === 'offline' || location === 'web';
  };

  export const avatarUrl = (friend: LimitedUserFriend) =>
    friend.profilePicOverrideThumbnail ||
    friend.currentAvatarThumbnailImageUrl ||
    friend.imageUrl ||
    '';

  export const currentUserAvatarUrl = (user: CurrentUser) =>
    user.profilePicOverrideThumbnail ||
    user.currentAvatarThumbnailImageUrl ||
    user.userIcon ||
    user.imageUrl ||
    user.currentAvatarImageUrl ||
    '';

  export type UserSummary = LimitedUserFriend | CurrentUser;
  export type RgbColor = [number, number, number];

  const isFriend = (user: UserSummary): user is LimitedUserFriend =>
    'isFriend' in user || 'friendKey' in user;

  export const resolveAvatarUrl = (user: UserSummary) =>
    isFriend(user) ? avatarUrl(user) : currentUserAvatarUrl(user);

  export const statusKey = (friend: LimitedUserFriend) =>
    isOffline(friend) ? 'offline' : friend.status.toLowerCase();

  export const statusLabel = (statusKey: string) => {
    switch (statusKey) {
      case 'join me':
        return t('status.joinMe');
      case 'active':
        return t('status.active');
      case 'ask me':
        return t('status.askMe');
      case 'busy':
        return t('status.busy');
      default:
        return t('status.offline');
    }
  };

  export const statusColorClass = (statusKey: string) => {
    switch (statusKey) {
      case 'join me':
        return 'bg-vrc-join-me';
      case 'active':
        return 'bg-vrc-online';
      case 'ask me':
        return 'bg-vrc-ask-me';
      case 'busy':
        return 'bg-vrc-do-not-disturb';
      default:
        return 'bg-black';
    }
  };

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

  export const dominantColorForUrl = async (url: string) => {
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

  export const dominantColorForUser = async (user: UserSummary) =>
    dominantColorForUrl(resolveAvatarUrl(user));

  export const cardOverlayStyle = (rgb: RgbColor | null) => {
    if (!rgb) return {};
    const [r, g, b] = rgb;
    const accent = `rgba(${r}, ${g}, ${b}, ${ACCENT_ALPHA})`;
    const base = `rgba(${BASE_RGB.join(', ')}, ${BASE_ALPHA})`;
    return {
      backgroundColor: `rgba(${r}, ${g}, ${b}, ${CARD_ALPHA})`,
      backgroundImage: `linear-gradient(180deg, ${accent}, ${base})`,
    };
  };

  const STATUS_PRIORITY: Record<UserStatus, number> = {
    'join me': 0,
    active: 1,
    'ask me': 2,
    busy: 3,
    offline: 4,
  };

  const rankFriend = (friend: LimitedUserFriend) => {
    if (isOffline(friend)) return Number.POSITIVE_INFINITY;
    return STATUS_PRIORITY[friend.status] ?? Number.POSITIVE_INFINITY;
  };

  export const compareFriends = (
    first: LimitedUserFriend,
    second: LimitedUserFriend,
  ) => {
    const rankA = rankFriend(first);
    const rankB = rankFriend(second);
    if (rankA !== rankB) return rankA - rankB;
    return first.displayName
      .toLowerCase()
      .localeCompare(second.displayName.toLowerCase());
  };

  export const sortFriends = (friends: LimitedUserFriend[]) =>
    [...friends].sort(compareFriends);

  export const mergeFriends = (
    current: LimitedUserFriend[],
    next: LimitedUserFriend[],
  ) => {
    const byId = new Map(current.map((friend) => [friend.id, friend]));
    return next.map((friend) => {
      const existing = byId.get(friend.id);
      if (!existing) return friend;
      Object.assign(existing, friend);
      return existing;
    });
  };

  export const formatLastOnline = (friend: LimitedUserFriend) => {
    if (!isOffline(friend)) return null;

    const rawValue =
      friend.last_activity ??
      (friend as {lastActivity?: string}).lastActivity ??
      friend.last_login ??
      (friend as {lastLogin?: string}).lastLogin;

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
}
