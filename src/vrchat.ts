import {Temporal} from '@js-temporal/polyfill';
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

  export type EitherFriendOrCurrent = LimitedUserFriend | CurrentUser;

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

  export const resolveAvatarUrl = (user: EitherFriendOrCurrent) =>
    'isFriend' in user || 'friendKey' in user
      ? avatarUrl(user as LimitedUserFriend)
      : currentUserAvatarUrl(user as CurrentUser);

  export const statusKey = (friend: LimitedUserFriend) =>
    isOffline(friend) ? 'offline' : friend.status.toLowerCase();

  const STATUS_META: Record<
    UserStatus | 'offline',
    {labelKey: string; colorClass: string}
  > = {
    'join me': {labelKey: 'status.joinMe', colorClass: 'bg-vrc-join-me'},
    active: {labelKey: 'status.active', colorClass: 'bg-vrc-online'},
    'ask me': {labelKey: 'status.askMe', colorClass: 'bg-vrc-ask-me'},
    busy: {labelKey: 'status.busy', colorClass: 'bg-vrc-do-not-disturb'},
    offline: {labelKey: 'status.offline', colorClass: 'bg-black'},
  };

  export const statusLabel = (statusKey: string) => {
    const meta = STATUS_META[statusKey as UserStatus] ?? STATUS_META.offline;
    return t(meta.labelKey);
  };

  export const statusColorClass = (statusKey: string) => {
    const meta = STATUS_META[statusKey as UserStatus] ?? STATUS_META.offline;
    return meta.colorClass;
  };

  const STATUS_PRIORITY: Record<UserStatus, number> = {
    'join me': 0,
    active: 1,
    'ask me': 2,
    busy: 3,
    offline: 4,
  };

  export const compareFriends = (
    first: LimitedUserFriend,
    second: LimitedUserFriend,
  ) => {
    const rankA = isOffline(first)
      ? Number.POSITIVE_INFINITY
      : STATUS_PRIORITY[first.status] ?? Number.POSITIVE_INFINITY;
    const rankB = isOffline(second)
      ? Number.POSITIVE_INFINITY
      : STATUS_PRIORITY[second.status] ?? Number.POSITIVE_INFINITY;
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
