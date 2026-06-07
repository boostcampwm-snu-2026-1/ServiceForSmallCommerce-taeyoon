import { beforeEach, describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import LoginPage from '@/app/(auth)/login/page';
import { login } from '@/src/features/auth/api';
import { useAuthStore } from '@/src/features/auth/store';
import type { AuthResponse } from '@/src/features/auth/types';

const push = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push }),
}));

vi.mock('@/src/features/auth/api', () => ({
  login: vi.fn(),
}));

const mockLogin = vi.mocked(login);

beforeEach(() => {
  push.mockClear();
  mockLogin.mockReset();
  useAuthStore.setState({ token: null, user: null });
  localStorage.clear();
});

describe('LoginPage', () => {
  it('submits credentials, sets auth, and navigates to dashboard', async () => {
    const authResponse: AuthResponse = {
      token: 'tok-xyz',
      user: { id: 'u1', email: 'a@b.com', plan: 'free' },
    };
    mockLogin.mockResolvedValue(authResponse);

    render(<LoginPage />);

    fireEvent.change(screen.getByLabelText('이메일'), {
      target: { value: 'a@b.com' },
    });
    fireEvent.change(screen.getByLabelText('비밀번호'), {
      target: { value: 'password123' },
    });
    fireEvent.click(screen.getByRole('button', { name: '로그인' }));

    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledWith({
        email: 'a@b.com',
        password: 'password123',
      });
    });

    await waitFor(() => {
      expect(useAuthStore.getState().token).toBe('tok-xyz');
    });
    expect(push).toHaveBeenCalledWith('/dashboard');
  });
});
