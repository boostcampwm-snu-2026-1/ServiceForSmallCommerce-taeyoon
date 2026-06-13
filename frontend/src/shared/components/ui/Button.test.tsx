import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Button } from './Button';

describe('Button', () => {
  it('renders children', () => {
    render(<Button>클릭</Button>);
    expect(screen.getByRole('button', { name: '클릭' })).toBeInTheDocument();
  });

  it('applies primary variant classes by default', () => {
    render(<Button>버튼</Button>);
    expect(screen.getByRole('button')).toHaveClass('bg-brand-600');
  });

  it('applies outline variant classes', () => {
    render(<Button variant="outline">버튼</Button>);
    const btn = screen.getByRole('button');
    expect(btn).toHaveClass('border');
    expect(btn).not.toHaveClass('bg-brand-600');
  });

  it('applies size classes', () => {
    render(<Button size="lg">버튼</Button>);
    expect(screen.getByRole('button')).toHaveClass('text-base');
  });

  it('reflects disabled prop', () => {
    render(<Button disabled>버튼</Button>);
    expect(screen.getByRole('button')).toBeDisabled();
  });

  it('merges custom className', () => {
    render(<Button className="custom-x">버튼</Button>);
    expect(screen.getByRole('button')).toHaveClass('custom-x');
  });

  it('calls onClick', async () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>버튼</Button>);
    screen.getByRole('button').click();
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});
