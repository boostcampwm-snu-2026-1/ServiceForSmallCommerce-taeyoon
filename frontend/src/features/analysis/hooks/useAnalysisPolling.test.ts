import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { createElement } from 'react';
import type { ReactNode } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { pollInterval, useAnalysisPolling } from './useAnalysisPolling';
import { getAnalysis } from '@/src/features/analysis/api';
import type { Analysis } from '@/src/features/analysis/types';

vi.mock('@/src/features/analysis/api', () => ({
  getAnalysis: vi.fn(),
}));

const mockGet = vi.mocked(getAnalysis);

const completed: Analysis = {
  id: 'an-1',
  status: 'completed',
  urls: ['https://coupang.com/a'],
  result: {
    products: [],
    insights: {
      top_complaints: [],
      top_positives: [],
      improvement_points: [],
      competitor_weaknesses: [],
      purchase_drivers: [],
    },
  },
  error: null,
  created_at: '2026-06-07T00:00:00Z',
  completed_at: '2026-06-07T00:01:00Z',
};

function wrapper({ children }: { children: ReactNode }) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return createElement(QueryClientProvider, { client }, children);
}

beforeEach(() => {
  mockGet.mockReset();
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe('pollInterval', () => {
  it('polls every 2s while pending', () => {
    expect(pollInterval('pending')).toBe(2000);
  });

  it('polls every 2s while crawling', () => {
    expect(pollInterval('crawling')).toBe(2000);
  });

  it('polls every 2s while analyzing', () => {
    expect(pollInterval('analyzing')).toBe(2000);
  });

  it('stops polling when completed', () => {
    expect(pollInterval('completed')).toBe(false);
  });

  it('stops polling when failed', () => {
    expect(pollInterval('failed')).toBe(false);
  });

  it('polls every 2s when status is undefined', () => {
    expect(pollInterval(undefined)).toBe(2000);
  });
});

describe('useAnalysisPolling', () => {
  it('fetches the analysis and exposes the completed status', async () => {
    mockGet.mockResolvedValue(completed);

    const { result } = renderHook(() => useAnalysisPolling('an-1'), { wrapper });

    await waitFor(() => {
      expect(result.current.data?.status).toBe('completed');
    });
    expect(mockGet).toHaveBeenCalledWith('an-1');
  });
});
