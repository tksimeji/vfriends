import {computed, type Ref} from 'vue';
import type {VRChat} from '../vrchat.ts';

export const resolveFriendAvatarUrl = (friend: VRChat.LimitedUserFriend) =>
  friend.profilePicOverrideThumbnail ||
  friend.currentAvatarThumbnailImageUrl ||
  friend.imageUrl ||
  '';

export const resolveCurrentUserAvatarUrl = (user: VRChat.CurrentUser) =>
  user.profilePicOverrideThumbnail ||
  user.currentAvatarThumbnailImageUrl ||
  user.userIcon ||
  user.imageUrl ||
  user.currentAvatarImageUrl ||
  '';

export const resolveAvatarUrl = (user: VRChat.EitherFriendOrCurrent) =>
  'isFriend' in user || 'friendKey' in user
    ? resolveFriendAvatarUrl(user as VRChat.LimitedUserFriend)
    : resolveCurrentUserAvatarUrl(user as VRChat.CurrentUser);

export const useAvatarUrl = (
  user: Ref<VRChat.EitherFriendOrCurrent | null | undefined>,
) =>
  computed(() => (user.value ? resolveAvatarUrl(user.value) : ''));
