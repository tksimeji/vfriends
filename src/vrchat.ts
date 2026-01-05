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
}
