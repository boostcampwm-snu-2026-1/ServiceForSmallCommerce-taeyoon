import { beforeEach, describe, expect, it, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
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

describe('Home (root)', () => {
  it('renders the landing page when there is no token', async () => {
    render(<Home />);

    // 미인증 방문자에게는 랜딩 히어로/CTA가 보이고 리다이렉트가 일어나지 않는다.
    await waitFor(() => {
      expect(
        screen.getByRole('heading', {
          level: 1,
          name: '리뷰가 알려주는 경쟁사를 이기는 법',
        })
      ).toBeInTheDocument();
    });
    expect(replace).not.toHaveBeenCalled();
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
