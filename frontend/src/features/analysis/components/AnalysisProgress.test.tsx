import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { AnalysisProgress } from './AnalysisProgress';

describe('AnalysisProgress', () => {
  it('renders the three stepper labels', () => {
    render(<AnalysisProgress status="crawling" />);

    expect(screen.getByText('리뷰 수집')).toBeInTheDocument();
    expect(screen.getByText('AI 분석')).toBeInTheDocument();
    expect(screen.getByText('분석 완료')).toBeInTheDocument();
  });

  it('shows the waiting label when pending', () => {
    render(<AnalysisProgress status="pending" />);

    expect(screen.getByText('분석 대기 중')).toBeInTheDocument();
    expect(screen.getByText(/분석 중입니다/)).toBeInTheDocument();
  });

  it('shows the crawling label and guidance while crawling', () => {
    render(<AnalysisProgress status="crawling" />);

    expect(screen.getByText('리뷰 수집 중')).toBeInTheDocument();
    expect(screen.getByText(/분석 중입니다/)).toBeInTheDocument();
  });

  it('shows the analyzing label while analyzing', () => {
    render(<AnalysisProgress status="analyzing" />);

    expect(screen.getByText('AI 분석 중')).toBeInTheDocument();
    expect(screen.getByText(/분석 중입니다/)).toBeInTheDocument();
  });
});
