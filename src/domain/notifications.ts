export type NotificationConfig = {
  messageTemplate: string;
  sound: string;
};

export type FriendNotification = {
  enabled: boolean;
  useCustom?: boolean;
  messageTemplate?: string | null;
  sound?: string | null;
};

export type FriendNotificationPatch = {
  enabled?: boolean;
  useCustom?: boolean;
  messageTemplate?: string | null;
  sound?: string | null;
};

export type FriendNotificationMap = Record<string, FriendNotification>;
