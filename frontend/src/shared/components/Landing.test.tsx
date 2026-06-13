import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Landing } from './Landing';

describe('Landing', () => {
  it('renders the hero headline', () => {
    render(<Landing />);
    expect(
      screen.getByRole('heading', {
        level: 1,
        name: '리뷰가 알려주는 경쟁사를 이기는 법',
      })
    ).toBeInTheDocument();
  });

  it('renders register and login CTA links', () => {
    render(<Landing />);

    const registerLinks = screen.getAllByRole('link', {
      name: /무료로 시작/,
    });
    expect(registerLinks.length).toBeGreaterThan(0);
    expect(registerLinks[0]).toHaveAttribute('href', '/register');

    const loginLinks = screen.getAllByRole('link', { name: '로그인' });
    expect(loginLinks.length).toBeGreaterThan(0);
    expect(loginLinks[0]).toHaveAttribute('href', '/login');
  });

  it('renders the three pricing plans', () => {
    render(<Landing />);
    expect(
      screen.getByRole('heading', { level: 3, name: '무료' })
    ).toBeInTheDocument();
    expect(
      screen.getByRole('heading', { level: 3, name: '스타터' })
    ).toBeInTheDocument();
    expect(
      screen.getByRole('heading', { level: 3, name: '프로' })
    ).toBeInTheDocument();
  });

  it('renders the footer copyright', () => {
    render(<Landing />);
    expect(
      screen.getByText(/© 2026 Coupang Review AI. All rights reserved\./)
    ).toBeInTheDocument();
  });
});
