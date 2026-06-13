import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Badge } from './Badge';

describe('Badge', () => {
  it('renders children', () => {
    render(<Badge>완료</Badge>);
    expect(screen.getByText('완료')).toBeInTheDocument();
  });

  it('applies neutral variant by default', () => {
    render(<Badge>라벨</Badge>);
    expect(screen.getByText('라벨')).toHaveClass('bg-gray-100');
  });

  it('maps success variant class', () => {
    render(<Badge variant="success">성공</Badge>);
    expect(screen.getByText('성공')).toHaveClass('bg-green-100');
  });

  it('maps info variant to brand color', () => {
    render(<Badge variant="info">정보</Badge>);
    expect(screen.getByText('정보')).toHaveClass('bg-brand-50');
  });

  it('merges custom className', () => {
    render(<Badge className="ml-2">라벨</Badge>);
    expect(screen.getByText('라벨')).toHaveClass('ml-2');
  });
});
