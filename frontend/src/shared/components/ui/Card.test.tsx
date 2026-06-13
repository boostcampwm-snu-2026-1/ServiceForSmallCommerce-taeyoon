import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Card } from './Card';

describe('Card', () => {
  it('renders children', () => {
    render(<Card>내용</Card>);
    expect(screen.getByText('내용')).toBeInTheDocument();
  });

  it('applies base classes and merges className', () => {
    render(<Card className="p-6">내용</Card>);
    const el = screen.getByText('내용');
    expect(el).toHaveClass('rounded-lg');
    expect(el).toHaveClass('border');
    expect(el).toHaveClass('p-6');
  });
});
