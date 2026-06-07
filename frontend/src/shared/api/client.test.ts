import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { apiClient } from '@/src/shared/api/client';
import { useAuthStore } from '@/src/features/auth/store';

function mockFetchOk(body: unknown = {}) {
  const fetchMock = vi.fn().mockResolvedValue({
    ok: true,
    json: async () => body,
  });
  vi.stubGlobal('fetch', fetchMock);
  return fetchMock;
}

function headersOf(fetchMock: ReturnType<typeof vi.fn>): Record<string, string> {
  return fetchMock.mock.calls[0][1].headers as Record<string, string>;
}

beforeEach(() => {
  useAuthStore.setState({ token: null, user: null });
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe('apiClient auth header', () => {
  it('attaches Authorization header when a token exists', async () => {
    useAuthStore.setState({ token: 'tok-123', user: null });
    const fetchMock = mockFetchOk({ ok: true });

    await apiClient.get('/api/v1/users/me');

    expect(headersOf(fetchMock).Authorization).toBe('Bearer tok-123');
  });

  it('omits Authorization header when there is no token', async () => {
    const fetchMock = mockFetchOk({ token: 'x', user: null });

    await apiClient.post('/api/v1/auth/login', { email: 'a@b.com', password: 'p' });

    expect(headersOf(fetchMock).Authorization).toBeUndefined();
    expect(headersOf(fetchMock)['Content-Type']).toBe('application/json');
  });

  it('throws with the backend error message on non-ok response', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: false,
        statusText: 'Unauthorized',
        json: async () => ({ error: 'Unauthorized' }),
      }),
    );

    await expect(apiClient.get('/api/v1/analyses')).rejects.toThrow('Unauthorized');
  });
});
