import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { InsightReport } from './InsightReport';
import type { AnalysisResult } from '@/src/features/analysis/types';

const result: AnalysisResult = {
  products: [
    {
      url: 'https://coupang.com/competitor',
      product_name: '경쟁 청소기',
      total_reviews: 80,
      avg_rating: 4.0,
      rating_distribution: { '1': 4, '2': 4, '3': 8, '4': 30, '5': 34 },
      is_mine: false,
    },
    {
      url: 'https://coupang.com/a',
      product_name: '무선 청소기',
      total_reviews: 120,
      avg_rating: 4.3,
      rating_distribution: { '1': 5, '2': 5, '3': 10, '4': 40, '5': 60 },
      is_mine: true,
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
    comparison_summary: '내 제품은 평점이 높지만 배터리 불만이 더 두드러집니다.',
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

  it('renders the methodology sample size', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('분석 방법 · 데이터 출처')).toBeInTheDocument();
    expect(
      screen.getByText(/분석 표본: 총 200개 리뷰 \(상품 2개\)/),
    ).toBeInTheDocument();
  });

  it('renders a section description', () => {
    render(<InsightReport result={result} />);
    expect(
      screen.getByText(/내 상품이 파고들 기회입니다/),
    ).toBeInTheDocument();
  });

  it('renders the disclaimer', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText(/참고용 인사이트입니다/)).toBeInTheDocument();
  });

  it('renders the comparison summary when present', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('비교 총평')).toBeInTheDocument();
    expect(
      screen.getByText('내 제품은 평점이 높지만 배터리 불만이 더 두드러집니다.'),
    ).toBeInTheDocument();
  });

  it('omits the comparison summary card when null', () => {
    const noSummary: AnalysisResult = {
      ...result,
      insights: { ...result.insights, comparison_summary: null },
    };
    render(<InsightReport result={noSummary} />);
    expect(screen.queryByText('비교 총평')).not.toBeInTheDocument();
  });

  it('renders the my-product badge in the rating distribution', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('내 제품')).toBeInTheDocument();
    expect(screen.getByText('경쟁사')).toBeInTheDocument();
  });

  it('keeps the preserved section titles', () => {
    render(<InsightReport result={result} />);
    expect(screen.getByText('고쳐야 할 것 TOP 3')).toBeInTheDocument();
    expect(screen.getByText('경쟁사 약점')).toBeInTheDocument();
    expect(screen.getByText('구매 결정 요인')).toBeInTheDocument();
  });
});
