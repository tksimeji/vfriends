import {invoke} from '@tauri-apps/api/core';
import {AppSettings, FriendSettings} from './types.ts';
import type {VRChat} from './vrchat.ts';

export const fetchFriends = async (): Promise<VRChat.LimitedUserFriend[]> =>
  invoke<VRChat.LimitedUserFriend[]>('fetch_friends');

export const fetchWorld = async (worldId: string): Promise<VRChat.World> =>
  invoke<VRChat.World>('fetch_world', {worldId});

export const fetchIconDataUri = async (url: string) => {
  if (!url) return null;
  return invoke<string | null>('fetch_icon_data_uri', {url});
};

export const beginAuth = async (username: string, password: string) =>
  invoke('begin_auth', {username, password});

export const verifyTwoFactor = async (twoFactorCode: string, twoFactorMethod: string) =>
  invoke('verify_two_factor', {twoFactorCode, twoFactorMethod});

export const restoreSession = async () =>
  invoke<VRChat.CurrentUser | null>('restore_session');

export const logout = async () => invoke('logout');

export const fetchFriendSettings = () =>
  invoke<Record<string, FriendSettings>>('fetch_friend_settings');

export const setFriendSettings = (
  friendId: string,
  patch: Partial<FriendSettings>,
) =>
  invoke('set_friend_settings', {
    friendId,
    patch,
  });

export const fetchAppSettings = () =>
  invoke<AppSettings>('fetch_app_settings');

export const setAppSettings = (settings: Partial<AppSettings>) => {
  const payload: Partial<AppSettings> = {};
  if ('defaultMessage' in settings) {
    payload.defaultMessage = settings.defaultMessage;
  }
  if ('defaultSound' in settings) {
    payload.defaultSound = settings.defaultSound;
  }
  return invoke<AppSettings>('set_app_settings', {
    settings: payload,
  });
};

export const previewNotificationSound = (sound: string | null) =>
  invoke<number | null>('preview_notification_sound', {
    sound,
  });

export const saveNotificationSound = (name: string, bytes: number[]) =>
  invoke<string>('save_notification_sound', {
    name,
    bytes,
  });

export const saveNotificationSoundPath = (path: string) =>
  invoke<string>('save_notification_sound_path', {
    path,
  });
