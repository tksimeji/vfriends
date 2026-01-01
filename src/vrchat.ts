export namespace VRChat {
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

  export const statusKey = (friend: LimitedUserFriend) =>
    isOffline(friend) ? 'offline' : friend.status.toLowerCase();

  export const statusLabel = (statusKey: string) => {
    switch (statusKey) {
      case 'join me':
        return 'だれでもおいで';
      case 'active':
        return 'オンライン';
      case 'ask me':
        return 'きいてみてね';
      case 'busy':
        return '取り込み中';
      default:
        return 'オフライン';
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
}
