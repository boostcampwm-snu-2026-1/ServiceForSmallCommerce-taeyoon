import { afterEach, describe, expect, it, vi } from 'vitest';
import { createAnalysis, getAnalysis } from './index';

afterEach(() => {
  vi.restoreAllMocks();
});

function mockFetch(body: unknown, ok = true, status = 200) {
  vi.stubGlobal(
    'fetch',
    vi.fn().mockResolvedValue({
      ok,
      status,
      statusText: 'OK',
      json: async () => body,
    }),
  );
}

describe('analysis api', () => {
  it('createAnalysis posts my_url and competitor urls and returns analysis id', async () => {
    mockFetch({ analysis_id: 'abc', status: 'pending', created_at: '2026-06-07' });

    const res = await createAnalysis({
      my_url: 'https://www.coupang.com/vp/products/0',
      competitor_urls: ['https://www.coupang.com/vp/products/1'],
      review_limit: 50,
    });

    expect(res.analysis_id).toBe('abc');
    expect(res.status).toBe('pending');
    expect(fetch).toHaveBeenCalledOnce();
  });

  it('getAnalysis throws on non-ok response', async () => {
    mockFetch({ error: 'not found' }, false, 404);
    await expect(getAnalysis('missing')).rejects.toThrow('not found');
  });
});
