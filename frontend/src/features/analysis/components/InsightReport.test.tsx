import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { InsightReport } from './InsightReport';
import type { AnalysisResult } from '@/src/features/analysis/types';

const result: AnalysisResult = {
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
    competitor_weaknesses: [
      { title: '느린 충전', opportunity: '고속 충전으로 차별화' },
    ],
    purchase_drivers: ['가성비', '디자인'],
  },
};

describe('InsightReport', () => {
  it('renders the top improvement points', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('배터리 수명 개선')).toBeInTheDocument();
    expect(screen.getByText('무게 감소')).toBeInTheDocument();
    expect(screen.getByText('소음 저감')).toBeInTheDocument();
  });

  it('renders competitor weaknesses', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('느린 충전')).toBeInTheDocument();
    expect(screen.getByText('고속 충전으로 차별화')).toBeInTheDocument();
  });

  it('renders purchase drivers', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('가성비')).toBeInTheDocument();
    expect(screen.getByText('디자인')).toBeInTheDocument();
  });

  it('renders complaints and positives', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('배터리가 빨리 닳음')).toBeInTheDocument();
    expect(screen.getByText('흡입력이 강함')).toBeInTheDocument();
  });

  it('renders product name and rating', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('무선 청소기')).toBeInTheDocument();
    expect(screen.getByText(/4\.3/)).toBeInTheDocument();
  });
});
