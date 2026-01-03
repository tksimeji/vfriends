import {invoke} from '@tauri-apps/api/core';
import type {
  FriendNotificationMap,
  FriendNotificationPatch,
  NotificationConfig,
} from '../domain/notifications';

export const fetchNotificationPreferences = () =>
  invoke<FriendNotificationMap>('fetch_notification_preferences');

export const setNotificationPreference = (
  friendId: string,
  patch: FriendNotificationPatch,
) =>
  invoke('set_notification_preference', {
    friendId,
    patch,
  });

export const fetchNotificationSettings = () =>
  invoke<NotificationConfig>('fetch_notification_settings');

export const setNotificationSettings = (settings: NotificationConfig) =>
  invoke('set_notification_settings', {
    settings,
  });

export const previewNotificationSound = (sound: string | null) =>
  invoke('preview_notification_sound', {
    sound,
  });

export const saveNotificationSound = (name: string, bytes: number[]) =>
  invoke<string>('save_notification_sound', {
    name,
    bytes,
  });
