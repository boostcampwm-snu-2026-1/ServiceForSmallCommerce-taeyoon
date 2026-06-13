import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Wordmark } from './Wordmark';

describe('Wordmark', () => {
  it('renders brand text', () => {
    render(<Wordmark />);
    expect(screen.getByText(/Coupang Review/)).toBeInTheDocument();
    expect(screen.getByText('AI')).toBeInTheDocument();
  });

  it('merges custom className', () => {
    const { container } = render(<Wordmark className="text-lg" />);
    expect(container.firstChild).toHaveClass('text-lg');
    expect(container.firstChild).toHaveClass('inline-flex');
  });
});
