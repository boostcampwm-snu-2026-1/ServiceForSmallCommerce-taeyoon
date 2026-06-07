import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { AnalysisCard } from './AnalysisCard';
import type { Analysis } from '@/src/features/analysis/types';

type CardAnalysis = Pick<Analysis, 'id' | 'status' | 'urls' | 'created_at'>;

const base: CardAnalysis = {
  id: 'an-123',
  status: 'completed',
  urls: ['https://coupang.com/a'],
  created_at: '2026-06-07T00:00:00Z',
};

describe('AnalysisCard', () => {
  it('renders the status text and the first url', () => {
    render(<AnalysisCard analysis={base} />);
    expect(screen.getByText('완료')).toBeInTheDocument();
    expect(screen.getByText(/coupang\.com\/a/)).toBeInTheDocument();
  });

  it('summarizes multiple urls', () => {
    render(
      <AnalysisCard
        analysis={{ ...base, urls: ['https://a.com', 'https://b.com', 'https://c.com'] }}
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
