'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useQuery } from '@tanstack/react-query';
import { createAnalysis, listAnalyses } from '@/src/features/analysis/api';
import { UrlInput } from '@/src/features/analysis/components/UrlInput';
import { AnalysisCard } from '@/src/features/analysis/components/AnalysisCard';

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
    <div className="flex flex-col gap-8">
      <section className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
        <h1 className="mb-4 text-xl font-bold text-gray-900">경쟁 상품 분석</h1>
        <UrlInput onSubmit={handleSubmit} loading={submitting} />
        {error && <p className="mt-3 text-sm text-red-600">{error}</p>}
      </section>

      <section>
        <h2 className="mb-3 text-lg font-semibold text-gray-900">최근 분석</h2>
        {isLoading ? (
          <p className="text-sm text-gray-500">불러오는 중...</p>
        ) : analyses.length === 0 ? (
          <p className="text-sm text-gray-500">아직 분석 기록이 없습니다.</p>
        ) : (
          <div className="grid gap-3 sm:grid-cols-2">
            {analyses.map((analysis) => (
              <AnalysisCard key={analysis.id} analysis={analysis} />
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
