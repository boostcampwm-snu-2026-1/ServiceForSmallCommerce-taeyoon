import { beforeEach, describe, expect, it } from 'vitest';
import { useAuthStore } from './store';
import type { User } from './types';

const sampleUser: User = { id: 'u1', email: 'a@b.com', plan: 'free' };

beforeEach(() => {
  useAuthStore.setState({ token: null, user: null });
  localStorage.clear();
});

describe('auth store', () => {
  it('setAuth reflects token and user in state', () => {
    useAuthStore.getState().setAuth('tok123', sampleUser);

    expect(useAuthStore.getState().token).toBe('tok123');
    expect(useAuthStore.getState().user).toEqual(sampleUser);
  });

  it('logout clears token and user to null', () => {
    useAuthStore.getState().setAuth('tok123', sampleUser);
    useAuthStore.getState().logout();

    expect(useAuthStore.getState().token).toBeNull();
    expect(useAuthStore.getState().user).toBeNull();
  });

  it('setAuth persists data to localStorage', () => {
    useAuthStore.getState().setAuth('tok123', sampleUser);

    expect(localStorage.getItem('auth-storage')).toBeTruthy();
  });
});
