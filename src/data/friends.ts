import {invoke} from '@tauri-apps/api/core';
import type {VRChat} from '../vrchat.ts';

export const fetchFriends = () => invoke<VRChat.LimitedUserFriend[]>('fetch_friends');
