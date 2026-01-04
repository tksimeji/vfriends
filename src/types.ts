export type AppSettings = {
  defaultMessage: string;
  defaultSound: string | null;
  friendSettings: Record<string, FriendSettings>;
};

export type FriendSettings = {
  enabled: boolean;
  useOverride?: boolean;
  messageOverride?: string | null;
  soundOverride?: string | null;
};
