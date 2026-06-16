import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { AnalysisCard } from './AnalysisCard';
import type { Analysis } from '@/src/features/analysis/types';

type CardAnalysis = Pick<
  Analysis,
  'id' | 'status' | 'urls' | 'my_url' | 'created_at'
>;

const base: CardAnalysis = {
  id: 'an-123',
  status: 'completed',
  urls: ['https://coupang.com/a'],
  my_url: 'https://coupang.com/mine',
  created_at: '2026-06-07T00:00:00Z',
};

describe('AnalysisCard', () => {
  it('renders the status text and the my-product vs competitor summary', () => {
    render(<AnalysisCard analysis={base} />);
    expect(screen.getByText('완료')).toBeInTheDocument();
    expect(screen.getByText('내 제품 vs 경쟁 1개')).toBeInTheDocument();
  });

  it('falls back to summarizing urls when my_url is null', () => {
    render(<AnalysisCard analysis={{ ...base, my_url: null }} />);
    expect(screen.getByText(/coupang\.com\/a/)).toBeInTheDocument();
  });

  it('summarizes multiple urls when my_url is null', () => {
    render(
      <AnalysisCard
        analysis={{
          ...base,
          my_url: null,
          urls: ['https://a.com', 'https://b.com', 'https://c.com'],
        }}
      />,
    );
    expect(screen.getByText(/외 2개/)).toBeInTheDocument();
  });

  it('links to the analysis detail page', () => {
    render(<AnalysisCard analysis={base} />);
    const link = screen.getByRole('link');
    expect(link).toHaveAttribute('href', '/analyses/an-123');
  });

  it('renders failed status', () => {
    render(<AnalysisCard analysis={{ ...base, status: 'failed' }} />);
    expect(screen.getByText('실패')).toBeInTheDocument();
  });
});
