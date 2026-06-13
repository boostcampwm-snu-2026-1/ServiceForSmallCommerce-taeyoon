import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Footer } from './Footer';

describe('Footer', () => {
  it('renders copyright text', () => {
    render(<Footer />);
    expect(
      screen.getByText(/© 2026 Coupang Review AI. All rights reserved\./)
    ).toBeInTheDocument();
  });

  it('renders the one-line introduction', () => {
    render(<Footer />);
    expect(
      screen.getByText(
        '쿠팡 경쟁 상품 리뷰를 AI가 분석해 개선 인사이트를 제공합니다.'
      )
    ).toBeInTheDocument();
  });

  it('renders legal links', () => {
    render(<Footer />);

    const terms = screen.getByRole('link', { name: '이용약관' });
    expect(terms).toBeInTheDocument();
    expect(terms).toHaveAttribute('href', '/terms');

    const privacy = screen.getByRole('link', { name: '개인정보 처리방침' });
    expect(privacy).toBeInTheDocument();
    expect(privacy).toHaveAttribute('href', '/privacy');
  });
});
