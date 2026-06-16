'use client';

import { useState } from 'react';
import { Button, Input } from '@/src/shared/components/ui';

const MAX_COMPETITORS = 3;
const REVIEW_LIMIT_OPTIONS = [50, 100, 200, 500] as const;

interface UrlInputProps {
  onSubmit: (myUrl: string, competitorUrls: string[], reviewLimit: number) => void;
  loading?: boolean;
}

export function UrlInput({ onSubmit, loading = false }: UrlInputProps) {
  const [myUrl, setMyUrl] = useState<string>('');
  const [competitorUrls, setCompetitorUrls] = useState<string[]>(['']);
  const [reviewLimit, setReviewLimit] = useState<number>(100);

  function updateCompetitor(index: number, value: string) {
    setCompetitorUrls((prev) => prev.map((u, i) => (i === index ? value : u)));
  }

  function addCompetitor() {
    setCompetitorUrls((prev) =>
      prev.length < MAX_COMPETITORS ? [...prev, ''] : prev,
    );
  }

  function removeCompetitor(index: number) {
    setCompetitorUrls((prev) =>
      prev.length > 1 ? prev.filter((_, i) => i !== index) : prev,
    );
  }

  const trimmedMyUrl = myUrl.trim();
  const filteredCompetitors = competitorUrls
    .map((u) => u.trim())
    .filter((u) => u.length > 0);
  const canSubmit =
    !loading && trimmedMyUrl.length > 0 && filteredCompetitors.length > 0;

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!canSubmit) return;
    onSubmit(trimmedMyUrl, filteredCompetitors, reviewLimit);
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-5">
      <div className="flex flex-col gap-2 rounded-lg border border-brand-200 bg-brand-50/60 p-3.5">
        <label
          htmlFor="my-product-url"
          className="text-sm font-semibold text-brand-800"
        >
          내 제품 URL
        </label>
        <Input
          id="my-product-url"
          type="url"
          value={myUrl}
          onChange={(e) => setMyUrl(e.target.value)}
          placeholder="내 상품 쿠팡 URL"
          aria-label="내 제품 URL"
          className="min-w-0 flex-1"
        />
      </div>

      <div className="flex flex-col gap-2.5">
        <span className="text-sm font-semibold text-gray-700">경쟁 상품 URL</span>
        {competitorUrls.map((url, index) => (
          <div key={index} className="flex items-center gap-2">
            <Input
              type="url"
              value={url}
              onChange={(e) => updateCompetitor(index, e.target.value)}
              placeholder="경쟁 상품 쿠팡 URL"
              aria-label={`경쟁 상품 URL ${index + 1}`}
              className="min-w-0 flex-1"
            />
            <button
              type="button"
              onClick={() => removeCompetitor(index)}
              disabled={competitorUrls.length <= 1}
              aria-label={`경쟁 상품 URL ${index + 1} 삭제`}
              className="flex h-9 w-9 shrink-0 items-center justify-center rounded-md border border-gray-300 text-gray-400 transition hover:border-gray-400 hover:bg-gray-50 hover:text-gray-600 focus:border-brand-500 focus:outline-none focus:ring-2 focus:ring-brand-200 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:border-gray-300 disabled:hover:bg-transparent disabled:hover:text-gray-400"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth={2}
                strokeLinecap="round"
                strokeLinejoin="round"
                className="h-4 w-4"
                aria-hidden="true"
              >
                <path d="M18 6 6 18" />
                <path d="m6 6 12 12" />
              </svg>
            </button>
          </div>
        ))}

        <button
          type="button"
          onClick={addCompetitor}
          disabled={competitorUrls.length >= MAX_COMPETITORS}
          className="inline-flex items-center justify-center gap-1.5 self-start rounded-md border border-dashed border-gray-300 px-3 py-2 text-sm font-medium text-gray-600 transition hover:border-brand-400 hover:text-brand-600 focus:border-brand-500 focus:outline-none focus:ring-2 focus:ring-brand-200 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:border-gray-300 disabled:hover:text-gray-600"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth={2}
            strokeLinecap="round"
            strokeLinejoin="round"
            className="h-4 w-4"
            aria-hidden="true"
          >
            <path d="M12 5v14" />
            <path d="M5 12h14" />
          </svg>
          경쟁 상품 추가
        </button>
      </div>

      <label className="flex flex-col gap-1.5 text-sm font-medium text-gray-700">
        분석할 리뷰 수
        <select
          value={reviewLimit}
          onChange={(e) => setReviewLimit(Number(e.target.value))}
          aria-label="분석할 리뷰 수"
          className="w-full rounded-md border border-gray-300 px-3 py-2 text-gray-900 focus:border-brand-500 focus:outline-none focus:ring-2 focus:ring-brand-200 sm:w-48"
        >
          {REVIEW_LIMIT_OPTIONS.map((opt) => (
            <option key={opt} value={opt}>
              {opt}개
            </option>
          ))}
        </select>
      </label>

      <Button type="submit" disabled={!canSubmit} className="mt-1 w-full sm:w-auto sm:self-start">
        {loading ? '분석 중...' : '분석 시작'}
      </Button>
    </form>
  );
}
