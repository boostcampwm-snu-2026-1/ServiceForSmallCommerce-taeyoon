import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Container } from './Container';

describe('Container', () => {
  it('renders children', () => {
    render(<Container>내용</Container>);
    expect(screen.getByText('내용')).toBeInTheDocument();
  });

  it('applies base classes and merges className', () => {
    render(<Container className="py-8">내용</Container>);
    const el = screen.getByText('내용');
    expect(el).toHaveClass('mx-auto');
    expect(el).toHaveClass('max-w-5xl');
    expect(el).toHaveClass('py-8');
  });
});
