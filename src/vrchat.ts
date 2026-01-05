// @GeneratedFrom
// vrchatapi 1.20.6 - https://crates.io/crates/vrchatapi/1.20.6
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
   * Represents the `models::World` structure of the VRChat API's rust SDK.
   */
  export type World = {
    authorId: string;
    authorName: string;
    capacity: number;
    createdAt: string;
    defaultContentSettings?: InstanceContentSettings;
    description: string;
    favorites?: number;
    featured: boolean;
    heat: number;
    id: string;
    imageUrl: string;
    labsPublicationDate: string;
    name: string;
    namespace?: string;
    occupants?: number;
    organization: string;
    popularity: number;
    previewYoutubeId?: string | null;
    publicOccupants?: number;
    recommendedCapacity: number;
    releaseStatus: ReleaseStatus;
    storeId?: string;
    tags: string[];
    thumbnailImageUrl: string;
    updatedAt: string;
    urlList?: string[];
    version: number;
    visits: number;
  };

  /**
   * Represents the `models::InstanceContentSettings` structure of the VRChat API's rust SDK.
   */
  export type InstanceContentSettings = {
    drones?: boolean;
    emoji?: boolean;
    pedestals?: boolean;
    prints?: boolean;
    props?: boolean;
    stickers?: boolean;
  }

  /**
   * Represents the `model::DeveloperType` enum of the VRChat API's rust SDK.
   */
  export type DeveloperType = 'internal' | 'moderator' | 'none' | 'trusted';

  /**
   * Represents the `model::UserStatus` enum of the VRChat API's rust SDK.
   */
  export type UserStatus = 'active' | 'ask me' | 'busy' | 'join me' | 'offline';

  /**
   * Represents the `model::ReleaseStatus` enum of the VRChat API's rust SDK.
   */
  export type ReleaseStatus = 'all' | 'hidden' | 'private' | 'public';

  export type EitherFriendOrCurrent = LimitedUserFriend | CurrentUser;
}
