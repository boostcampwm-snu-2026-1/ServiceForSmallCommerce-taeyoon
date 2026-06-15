'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useQuery } from '@tanstack/react-query';
import { createAnalysis, listAnalyses } from '@/src/features/analysis/api';
import { UrlInput } from '@/src/features/analysis/components/UrlInput';
import { AnalysisCard } from '@/src/features/analysis/components/AnalysisCard';
import { EmptyState } from '@/src/shared/components/EmptyState';

export default function DashboardPage() {
  const router = useRouter();
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const { data, isLoading } = useQuery({
    queryKey: ['analyses'],
    queryFn: () => listAnalyses(),
  });

  async function handleSubmit(urls: string[], reviewLimit: number) {
    setError(null);
    setSubmitting(true);
    try {
      const res = await createAnalysis({ urls, review_limit: reviewLimit });
      router.push(`/analyses/${res.analysis_id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : '분석 생성에 실패했습니다.');
      setSubmitting(false);
    }
  }

  const analyses = data?.analyses ?? [];

  return (
    <div className="flex flex-col gap-8 sm:gap-10">
      <div className="overflow-hidden rounded-2xl bg-gradient-to-br from-brand-700 to-brand-900 p-6 shadow-lg sm:p-8">
        <div className="flex items-center gap-2.5">
          <span className="flex h-9 w-9 items-center justify-center rounded-lg bg-white/10 text-white ring-1 ring-inset ring-white/20">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth={2}
              strokeLinecap="round"
              strokeLinejoin="round"
              className="h-5 w-5"
              aria-hidden="true"
            >
              <circle cx="11" cy="11" r="7" />
              <path d="m21 21-4.3-4.3" />
            </svg>
          </span>
          <h1 className="text-xl font-bold text-white sm:text-2xl">경쟁 상품 분석</h1>
        </div>
        <p className="mt-3 max-w-2xl text-sm leading-relaxed text-brand-100">
          경쟁 상품의 쿠팡 URL을 입력하면 AI가 리뷰를 분석해 개선 포인트를 알려드립니다.
        </p>
        <div className="mt-5 rounded-xl bg-white p-4 shadow-sm sm:p-5">
          <UrlInput onSubmit={handleSubmit} loading={submitting} />
          {error && <p className="mt-3 text-sm text-red-600">{error}</p>}
        </div>
      </div>

      <section>
        <div className="mb-4 flex items-center justify-between gap-2">
          <h2 className="text-lg font-semibold text-gray-900">최근 분석</h2>
          {!isLoading && analyses.length > 0 && (
            <span className="rounded-full bg-brand-50 px-2.5 py-0.5 text-xs font-medium text-brand-700">
              총 {analyses.length}건
            </span>
          )}
        </div>
        {isLoading ? (
          <p className="text-sm text-gray-500">불러오는 중...</p>
        ) : analyses.length === 0 ? (
          <EmptyState
            title="아직 분석 기록이 없습니다."
            description="위에서 경쟁 상품 URL을 입력해 첫 분석을 시작해 보세요."
          />
        ) : (
          <div className="grid gap-4 sm:grid-cols-2">
            {analyses.map((analysis) => (
              <AnalysisCard key={analysis.id} analysis={analysis} />
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
