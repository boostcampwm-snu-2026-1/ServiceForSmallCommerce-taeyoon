'use client';

import Link from 'next/link';
import { useParams } from 'next/navigation';
import { useAnalysisPolling } from '@/src/features/analysis/hooks/useAnalysisPolling';
import { InsightReport } from '@/src/features/analysis/components/InsightReport';
import { AnalysisProgress } from '@/src/features/analysis/components/AnalysisProgress';
import type { Analysis, AnalysisResult } from '@/src/features/analysis/types';

function BackLink() {
  return (
    <Link
      href="/dashboard"
      className="inline-flex items-center gap-1 self-start rounded-lg px-3 py-1.5 text-sm text-gray-600 transition hover:bg-gray-100 hover:text-brand-700"
    >
      <svg viewBox="0 0 20 20" fill="currentColor" className="h-4 w-4" aria-hidden="true">
        <path fillRule="evenodd" d="M12.7 4.3a1 1 0 010 1.4L8.42 10l4.29 4.3a1 1 0 01-1.42 1.4l-5-5a1 1 0 010-1.4l5-5a1 1 0 011.42 0z" clipRule="evenodd" />
      </svg>
      대시보드로 돌아가기
    </Link>
  );
}

function formatCompletedAt(iso: string): string {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return iso;
  return date.toLocaleString('ko-KR');
}

function averageRating(products: AnalysisResult['products']): number | null {
  if (products.length === 0) return null;
  const sum = products.reduce((acc, product) => acc + product.avg_rating, 0);
  return Math.round((sum / products.length) * 10) / 10;
}

function ResultHeader({
  result,
  completedAt,
}: {
  result: AnalysisResult;
  completedAt: string | null;
}) {
  const totalReviews = result.products.reduce(
    (sum, product) => sum + product.total_reviews,
    0,
  );
  const myProducts = result.products.filter((p) => p.is_mine);
  const competitorProducts = result.products.filter((p) => !p.is_mine);
  const myAvg = averageRating(myProducts);
  const competitorAvg = averageRating(competitorProducts);

  return (
    <div className="rounded-2xl bg-gradient-to-br from-brand-700 to-brand-900 p-6 text-white shadow-md">
      <p className="text-sm font-semibold text-brand-100">분석 완료</p>

      <div className="mt-5 grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div className="rounded-xl bg-white/10 p-4 ring-1 ring-inset ring-white/15">
          <p className="text-xs font-medium text-brand-100">내 제품 평균 평점</p>
          <p className="mt-1 text-2xl font-bold leading-none sm:text-3xl">
            {myAvg !== null ? myAvg : '—'}
          </p>
        </div>
        <div className="rounded-xl bg-white/10 p-4 ring-1 ring-inset ring-white/15">
          <p className="text-xs font-medium text-brand-100">경쟁사 평균 평점</p>
          <p className="mt-1 text-2xl font-bold leading-none sm:text-3xl">
            {competitorAvg !== null ? competitorAvg : '—'}
          </p>
        </div>
      </div>

      <div className="mt-4 grid grid-cols-1 gap-5 sm:grid-cols-3 sm:gap-6">
        <div className="min-w-0">
          <p className="text-2xl font-bold leading-none sm:text-3xl">
            {result.products.length}
          </p>
          <p className="mt-2 text-xs font-medium text-brand-100">분석 상품</p>
        </div>
        <div className="min-w-0">
          <p className="text-2xl font-bold leading-none sm:text-3xl">{totalReviews}</p>
          <p className="mt-2 text-xs font-medium text-brand-100">총 리뷰 수</p>
        </div>
        <div className="min-w-0">
          <p className="break-words text-sm font-semibold leading-snug sm:text-base">
            {completedAt ? formatCompletedAt(completedAt) : '—'}
          </p>
          <p className="mt-2 text-xs font-medium text-brand-100">생성 시각</p>
        </div>
      </div>
    </div>
  );
}

function WarningIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-5 w-5" aria-hidden="true">
      <path d="M10.3 3.86 1.82 18a1 1 0 0 0 .86 1.5h18.64a1 1 0 0 0 .86-1.5L13.7 3.86a1 1 0 0 0-1.72 0z" />
      <path d="M12 9v4M12 17h.01" />
    </svg>
  );
}

function ErrorView({ message }: { message: string }) {
  return (
    <div className="rounded-2xl border border-red-200 bg-red-50 p-6">
      <div className="flex items-start gap-3">
        <span className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-red-100 text-red-600">
          <WarningIcon />
        </span>
        <div>
          <p className="font-semibold text-red-800">분석에 실패했습니다.</p>
          <p className="mt-1 text-sm text-red-700">{message}</p>
        </div>
      </div>
    </div>
  );
}

function CompletedView({ analysis }: { analysis: Analysis }) {
  if (!analysis.result) return null;
  return (
    <div className="flex flex-col gap-6">
      <ResultHeader result={analysis.result} completedAt={analysis.completed_at} />
      <InsightReport result={analysis.result} />
    </div>
  );
}

export default function AnalysisResultPage() {
  const params = useParams<{ id: string }>();
  const id = params?.id ?? '';
  const { data: analysis, isLoading, isError } = useAnalysisPolling(id);

  let content: React.ReactNode;
  if (isError) {
    content = <ErrorView message="분석 정보를 불러오지 못했습니다." />;
  } else if (isLoading || !analysis) {
    content = <AnalysisProgress status="pending" />;
  } else if (analysis.status === 'failed') {
    content = <ErrorView message={analysis.error ?? '알 수 없는 오류가 발생했습니다.'} />;
  } else if (analysis.status === 'completed' && analysis.result) {
    content = <CompletedView analysis={analysis} />;
  } else {
    content = <AnalysisProgress status={analysis.status} />;
  }

  return (
    <div className="flex flex-col gap-6">
      <BackLink />
      {content}
    </div>
  );
}
