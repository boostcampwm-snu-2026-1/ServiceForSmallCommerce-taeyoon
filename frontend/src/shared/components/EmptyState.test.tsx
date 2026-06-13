import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { EmptyState } from './EmptyState';

describe('EmptyState', () => {
  it('renders the title and description', () => {
    render(<EmptyState title="아직 분석 기록이 없습니다." description="첫 분석을 시작해 보세요." />);
    expect(screen.getByText('아직 분석 기록이 없습니다.')).toBeInTheDocument();
    expect(screen.getByText('첫 분석을 시작해 보세요.')).toBeInTheDocument();
  });

  it('renders the action node', () => {
    render(
      <EmptyState
        title="비어 있음"
        action={<button type="button">새로 만들기</button>}
      />,
    );
    expect(screen.getByRole('button', { name: '새로 만들기' })).toBeInTheDocument();
  });

  it('merges custom className', () => {
    const { container } = render(<EmptyState title="t" className="mt-8" />);
    expect(container.firstChild).toHaveClass('mt-8');
    expect(container.firstChild).toHaveClass('border-dashed');
  });
});
