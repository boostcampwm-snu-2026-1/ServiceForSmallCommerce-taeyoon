'use client';

import { useState } from 'react';
import { Button, Input } from '@/src/shared/components/ui';

const MAX_URLS = 3;
const REVIEW_LIMIT_OPTIONS = [50, 100, 200, 500] as const;

interface UrlInputProps {
  onSubmit: (urls: string[], reviewLimit: number) => void;
  loading?: boolean;
}

export function UrlInput({ onSubmit, loading = false }: UrlInputProps) {
  const [urls, setUrls] = useState<string[]>(['']);
  const [reviewLimit, setReviewLimit] = useState<number>(100);

  function updateUrl(index: number, value: string) {
    setUrls((prev) => prev.map((u, i) => (i === index ? value : u)));
  }

  function addUrl() {
    setUrls((prev) => (prev.length < MAX_URLS ? [...prev, ''] : prev));
  }

  function removeUrl(index: number) {
    setUrls((prev) => (prev.length > 1 ? prev.filter((_, i) => i !== index) : prev));
  }

  const filteredUrls = urls.map((u) => u.trim()).filter((u) => u.length > 0);
  const canSubmit = !loading && filteredUrls.length > 0;

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!canSubmit) return;
    onSubmit(filteredUrls, reviewLimit);
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <div className="flex flex-col gap-3">
        {urls.map((url, index) => (
          <div key={index} className="flex items-center gap-2">
            <Input
              type="url"
              value={url}
              onChange={(e) => updateUrl(index, e.target.value)}
              placeholder="경쟁 상품 쿠팡 URL"
              aria-label={`URL ${index + 1}`}
              className="flex-1"
            />
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={() => removeUrl(index)}
              disabled={urls.length <= 1}
              aria-label={`URL ${index + 1} 삭제`}
            >
              삭제
            </Button>
          </div>
        ))}
      </div>

      <Button
        type="button"
        variant="ghost"
        size="sm"
        onClick={addUrl}
        disabled={urls.length >= MAX_URLS}
        className="self-start border border-dashed border-gray-300"
      >
        URL 추가
      </Button>

      <label className="flex flex-col gap-1 text-sm font-medium text-gray-700">
        분석할 리뷰 수
        <select
          value={reviewLimit}
          onChange={(e) => setReviewLimit(Number(e.target.value))}
          aria-label="분석할 리뷰 수"
          className="rounded-md border border-gray-300 px-3 py-2 text-gray-900 focus:border-brand-500 focus:outline-none"
        >
          {REVIEW_LIMIT_OPTIONS.map((opt) => (
            <option key={opt} value={opt}>
              {opt}개
            </option>
          ))}
        </select>
      </label>

      <Button type="submit" disabled={!canSubmit} className="mt-2">
        {loading ? '분석 중...' : '분석 시작'}
      </Button>
    </form>
  );
}
