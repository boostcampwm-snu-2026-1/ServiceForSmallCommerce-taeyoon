import { beforeEach, describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ReactElement } from 'react';
import DashboardPage from '@/app/(dashboard)/dashboard/page';
import { createAnalysis, listAnalyses } from '@/src/features/analysis/api';

const push = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push }),
}));

vi.mock('@/src/features/analysis/api', () => ({
  createAnalysis: vi.fn(),
  listAnalyses: vi.fn(),
}));

const mockCreate = vi.mocked(createAnalysis);
const mockList = vi.mocked(listAnalyses);

function renderWithClient(ui: ReactElement) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(<QueryClientProvider client={client}>{ui}</QueryClientProvider>);
}

beforeEach(() => {
  push.mockClear();
  mockCreate.mockReset();
  mockList.mockReset();
  mockList.mockResolvedValue({ analyses: [], total: 0, page: 1, per_page: 20 });
});

describe('DashboardPage', () => {
  it('creates an analysis on submit and navigates to its detail page', async () => {
    mockCreate.mockResolvedValue({
      analysis_id: 'an-999',
      status: 'pending',
      created_at: '2026-06-07T00:00:00Z',
    });

    renderWithClient(<DashboardPage />);

    fireEvent.change(screen.getByLabelText('URL 1'), {
      target: { value: 'https://coupang.com/x' },
    });
    fireEvent.click(screen.getByRole('button', { name: '분석 시작' }));

    await waitFor(() => {
      expect(mockCreate).toHaveBeenCalledWith({
        urls: ['https://coupang.com/x'],
        review_limit: 100,
      });
    });

    await waitFor(() => {
      expect(push).toHaveBeenCalledWith('/analyses/an-999');
    });
  });

  it('shows an empty state when there is no history', async () => {
    renderWithClient(<DashboardPage />);
    await waitFor(() => {
      expect(screen.getByText('아직 분석 기록이 없습니다.')).toBeInTheDocument();
    });
  });
});
