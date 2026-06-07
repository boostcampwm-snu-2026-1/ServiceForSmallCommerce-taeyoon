import { beforeEach, describe, expect, it, vi } from 'vitest';
import { render, waitFor } from '@testing-library/react';
import Home from '@/app/page';
import { useAuthStore } from '@/src/features/auth/store';

const replace = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({ replace }),
}));

beforeEach(() => {
  replace.mockClear();
  useAuthStore.setState({ token: null, user: null });
});

describe('Home (root redirect)', () => {
  it('redirects to /login when there is no token', async () => {
    render(<Home />);
    await waitFor(() => {
      expect(replace).toHaveBeenCalledWith('/login');
    });
  });

  it('redirects to /dashboard when a token is present', async () => {
    useAuthStore.setState({
      token: 'tok-123',
      user: { id: 'u-1', email: 'a@b.com', plan: 'free' },
    });
    render(<Home />);
    await waitFor(() => {
      expect(replace).toHaveBeenCalledWith('/dashboard');
    });
  });
});
