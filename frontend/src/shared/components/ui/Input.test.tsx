import { describe, it, expect, vi } from 'vitest';
import { createRef } from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { Input } from './Input';

describe('Input', () => {
  it('renders with placeholder and value', () => {
    render(<Input placeholder="이메일" value="a@b.com" onChange={() => {}} />);
    const el = screen.getByPlaceholderText('이메일') as HTMLInputElement;
    expect(el).toBeInTheDocument();
    expect(el.value).toBe('a@b.com');
  });

  it('calls onChange', () => {
    const onChange = vi.fn();
    render(<Input placeholder="입력" onChange={onChange} />);
    fireEvent.change(screen.getByPlaceholderText('입력'), { target: { value: 'x' } });
    expect(onChange).toHaveBeenCalled();
  });

  it('forwards ref', () => {
    const ref = createRef<HTMLInputElement>();
    render(<Input ref={ref} placeholder="ref" />);
    expect(ref.current).toBeInstanceOf(HTMLInputElement);
  });

  it('merges custom className', () => {
    render(<Input placeholder="cls" className="mt-2" />);
    expect(screen.getByPlaceholderText('cls')).toHaveClass('mt-2');
  });
});
