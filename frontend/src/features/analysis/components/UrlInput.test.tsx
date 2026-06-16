import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/react';
import { UrlInput } from './UrlInput';

describe('UrlInput', () => {
  it('renders the my-product input and a single competitor input by default', () => {
    render(<UrlInput onSubmit={vi.fn()} />);
    expect(screen.getByLabelText('내 제품 URL')).toBeInTheDocument();
    expect(screen.getByLabelText('경쟁 상품 URL 1')).toBeInTheDocument();
    expect(screen.queryByLabelText('경쟁 상품 URL 2')).not.toBeInTheDocument();
  });

  it('adds competitor inputs up to a maximum of 3', () => {
    render(<UrlInput onSubmit={vi.fn()} />);
    const addButton = screen.getByRole('button', { name: '경쟁 상품 추가' });

    fireEvent.click(addButton);
    expect(screen.getByLabelText('경쟁 상품 URL 2')).toBeInTheDocument();

    fireEvent.click(addButton);
    expect(screen.getByLabelText('경쟁 상품 URL 3')).toBeInTheDocument();

    // capped at 3: button becomes disabled and no 4th input appears
    expect(addButton).toBeDisabled();
    fireEvent.click(addButton);
    expect(screen.queryByLabelText('경쟁 상품 URL 4')).not.toBeInTheDocument();
  });

  it('removes competitor inputs down to a minimum of 1', () => {
    render(<UrlInput onSubmit={vi.fn()} />);
    const addButton = screen.getByRole('button', { name: '경쟁 상품 추가' });
    fireEvent.click(addButton);
    expect(screen.getByLabelText('경쟁 상품 URL 2')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: '경쟁 상품 URL 2 삭제' }));
    expect(screen.queryByLabelText('경쟁 상품 URL 2')).not.toBeInTheDocument();

    // last remaining delete button is disabled
    expect(
      screen.getByRole('button', { name: '경쟁 상품 URL 1 삭제' }),
    ).toBeDisabled();
  });

  it('calls onSubmit with my url, competitor urls and the selected review limit', () => {
    const onSubmit = vi.fn();
    render(<UrlInput onSubmit={onSubmit} />);

    fireEvent.change(screen.getByLabelText('내 제품 URL'), {
      target: { value: 'https://coupang.com/mine' },
    });
    fireEvent.click(screen.getByRole('button', { name: '경쟁 상품 추가' }));
    fireEvent.change(screen.getByLabelText('경쟁 상품 URL 1'), {
      target: { value: 'https://coupang.com/a' },
    });
    // leave competitor 2 empty -> should be filtered out
    fireEvent.change(screen.getByLabelText('분석할 리뷰 수'), {
      target: { value: '200' },
    });

    fireEvent.click(screen.getByRole('button', { name: '분석 시작' }));

    expect(onSubmit).toHaveBeenCalledWith(
      'https://coupang.com/mine',
      ['https://coupang.com/a'],
      200,
    );
  });

  it('does not allow submit when my url is empty', () => {
    const onSubmit = vi.fn();
    render(<UrlInput onSubmit={onSubmit} />);
    fireEvent.change(screen.getByLabelText('경쟁 상품 URL 1'), {
      target: { value: 'https://coupang.com/a' },
    });
    const submit = screen.getByRole('button', { name: '분석 시작' });
    expect(submit).toBeDisabled();
    fireEvent.click(submit);
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it('does not allow submit when no competitor url is provided', () => {
    const onSubmit = vi.fn();
    render(<UrlInput onSubmit={onSubmit} />);
    fireEvent.change(screen.getByLabelText('내 제품 URL'), {
      target: { value: 'https://coupang.com/mine' },
    });
    const submit = screen.getByRole('button', { name: '분석 시작' });
    expect(submit).toBeDisabled();
    fireEvent.click(submit);
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it('disables submit and shows loading label while loading', () => {
    render(<UrlInput onSubmit={vi.fn()} loading />);
    const submit = screen.getByRole('button', { name: '분석 중...' });
    expect(submit).toBeDisabled();
  });
});
