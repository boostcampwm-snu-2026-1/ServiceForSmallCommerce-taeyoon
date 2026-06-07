import { beforeEach, describe, expect, it, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ReactElement } from 'react';
import AnalysisResultPage from '@/app/(dashboard)/analyses/[id]/page';
import { getAnalysis } from '@/src/features/analysis/api';
import type { Analysis } from '@/src/features/analysis/types';

vi.mock('next/navigation', () => ({
  useParams: () => ({ id: 'an-1' }),
}));

vi.mock('@/src/features/analysis/api', () => ({
  getAnalysis: vi.fn(),
}));

const mockGet = vi.mocked(getAnalysis);

const completed: Analysis = {
  id: 'an-1',
  status: 'completed',
  urls: ['https://coupang.com/a'],
  result: {
    products: [
      {
        url: 'https://coupang.com/a',
        product_name: '무선 청소기',
        total_reviews: 120,
        avg_rating: 4.3,
        rating_distribution: { '1': 5, '2': 5, '3': 10, '4': 40, '5': 60 },
      },
    ],
    insights: {
      top_complaints: [{ text: '배터리가 빨리 닳음', count: 30, severity: 'high' }],
      top_positives: [{ text: '흡입력이 강함', count: 50 }],
      improvement_points: [
        { rank: 1, title: '배터리 수명 개선', detail: '교체형 배터리 제공' },
        { rank: 2, title: '무게 감소', detail: '경량 소재 적용' },
        { rank: 3, title: '소음 저감', detail: '저소음 모터 적용' },
      ],
      competitor_weaknesses: [{ title: '느린 충전', opportunity: '고속 충전' }],
      purchase_drivers: ['가성비', '디자인'],
    },
  },
  error: null,
  created_at: '2026-06-07T00:00:00Z',
  completed_at: '2026-06-07T00:01:00Z',
};

const pending: Analysis = {
  id: 'an-1',
  status: 'pending',
  urls: ['https://coupang.com/a'],
  result: null,
  error: null,
  created_at: '2026-06-07T00:00:00Z',
  completed_at: null,
};

function renderWithClient(ui: ReactElement) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(<QueryClientProvider client={client}>{ui}</QueryClientProvider>);
}

beforeEach(() => {
  mockGet.mockReset();
});

describe('AnalysisResultPage', () => {
  it('renders the insight report when completed', async () => {
    mockGet.mockResolvedValue(completed);

    renderWithClient(<AnalysisResultPage />);

    await waitFor(() => {
      expect(screen.getByText('배터리 수명 개선')).toBeInTheDocument();
    });
    expect(screen.getByText('느린 충전')).toBeInTheDocument();
    expect(screen.getByText('무선 청소기')).toBeInTheDocument();
  });

  it('shows the progress UI while pending', async () => {
    mockGet.mockResolvedValue(pending);

    renderWithClient(<AnalysisResultPage />);

    await waitFor(() => {
      expect(screen.getByText('분석 대기 중')).toBeInTheDocument();
    });
    expect(screen.getByText(/분석 중입니다/)).toBeInTheDocument();
  });
});
